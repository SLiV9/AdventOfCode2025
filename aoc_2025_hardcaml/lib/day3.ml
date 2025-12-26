open! Core
open! Hardcaml
open! Signal

module I = struct
  type 'a t =
    { clock : 'a
    ; clear : 'a
    ; start : 'a
    ; finish : 'a
    ; data_in : 'a [@bits 8]
    ; data_in_valid : 'a
    }
  [@@deriving hardcaml]
end

module O = struct
  type 'a t =
    { part1 : 'a With_valid.t [@bits 64]
    ; part2 : 'a With_valid.t [@bits 64]
    }
  [@@deriving hardcaml]
end

module States = struct
  type t =
    | Idle
    | Processing
    | Done
  [@@deriving sexp_of, compare ~localize, enumerate]
end

let maximum (a : t) (b : t) : t = Signal.mux2 (a >: b) a b

let push_digit (num : t) (digit : t) : t =
  (uresize num ~width:56 *: of_string "8'd10") +: digit
;;

let create scope ({ clock; clear; start; finish; data_in; data_in_valid } : _ I.t) : _ O.t
  =
  let spec = Reg_spec.create ~clock ~clear () in
  let open Always in
  let sm = State_machine.create (module States) spec in
  let%hw_var is_digit = Variable.reg spec ~width:1 in
  let%hw_var digit = Variable.reg spec ~width:64 in
  let%hw_var digit_mask = Variable.reg spec ~width:64 in
  let%hw_var p1_sum = Variable.reg spec ~width:64 in
  let%hw_var p1_r0 = Variable.reg spec ~width:64 in
  let%hw_var p1_r1 = Variable.reg spec ~width:64 in
  let%hw_var p1_c0 = Variable.reg spec ~width:64 in
  let%hw_var p1_c1 = Variable.reg spec ~width:64 in
  (* let%hw_var p2_sum = Variable.reg spec ~width:64 in *)
  (* let%hw_var p2_r0 = Variable.reg spec ~width:64 in *)
  let part1 = Variable.wire ~default:(zero 64) () in
  let part1_valid = Variable.wire ~default:gnd () in
  let part2 = Variable.wire ~default:(zero 64) () in
  let part2_valid = Variable.wire ~default:gnd () in
  compile
    [ sm.switch
        [ ( Idle
          , [ when_
                start
                [ is_digit <-- zero 1
                ; digit <-- zero 64
                ; digit_mask <-- zero 64
                ; p1_sum <-- zero 64
                ; p1_r0 <-- zero 64
                ; p1_r1 <-- zero 64
                ; p1_c0 <-- zero 64
                ; p1_c1 <-- zero 64
                  (* ; p2_sum <-- zero 64 *)
                  (* ; p2_r0 <-- zero 64 *)
                ; sm.set_next Processing
                ]
            ] )
        ; ( Processing
          , [ when_
                data_in_valid
                [ is_digit <-- uresize (srl data_in ~by:4) ~width:1
                ; digit <-- uresize (uresize data_in ~width:4) ~width:64
                ; digit_mask <-- mux2 is_digit.value (ones 64) (zero 64)
                ; p1_sum <-- p1_sum.value +: mux2 is_digit.value (zero 64) p1_r0.value
                ; p1_c0 <-- push_digit p1_r1.value digit.value
                ; p1_c1 <-- digit.value
                ; p1_r0 <-- (digit_mask.value &: maximum p1_c0.value p1_r0.value)
                ; p1_r1 <-- (digit_mask.value &: maximum p1_c1.value p1_r1.value)
                ]
            ; when_ finish [ sm.set_next Done ]
            ] )
        ; ( Done
          , [ part1 <-- p1_sum.value +: p1_r0.value
            ; part1_valid <-- vdd
            ; part2 <-- zero 64 (* ; part2 <-- p2_sum.value +: p2_r0.value *)
            ; part2_valid <-- vdd
            ; when_ finish [ sm.set_next Processing ]
            ] )
        ]
    ];
  { part1 = { value = part1.value; valid = part1_valid.value }
  ; part2 = { value = part2.value; valid = part2_valid.value }
  }
;;

let hierarchical scope =
  let module Scoped = Hierarchy.In_scope (I) (O) in
  Scoped.hierarchical ~scope ~name:"day3" create
;;
