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
  let%hw_var p2_sum = Variable.reg spec ~width:64 in
  let%hw_var p2_r0 = Variable.reg spec ~width:64 in
  let%hw_var p2_r1 = Variable.reg spec ~width:64 in
  let%hw_var p2_r2 = Variable.reg spec ~width:64 in
  let%hw_var p2_r3 = Variable.reg spec ~width:64 in
  let%hw_var p2_r4 = Variable.reg spec ~width:64 in
  let%hw_var p2_r5 = Variable.reg spec ~width:64 in
  let%hw_var p2_r6 = Variable.reg spec ~width:64 in
  let%hw_var p2_r7 = Variable.reg spec ~width:64 in
  let%hw_var p2_r8 = Variable.reg spec ~width:64 in
  let%hw_var p2_r9 = Variable.reg spec ~width:64 in
  let%hw_var p2_rA = Variable.reg spec ~width:64 in
  let%hw_var p2_rB = Variable.reg spec ~width:64 in
  let%hw_var p2_c0 = Variable.reg spec ~width:64 in
  let%hw_var p2_c1 = Variable.reg spec ~width:64 in
  let%hw_var p2_c2 = Variable.reg spec ~width:64 in
  let%hw_var p2_c3 = Variable.reg spec ~width:64 in
  let%hw_var p2_c4 = Variable.reg spec ~width:64 in
  let%hw_var p2_c5 = Variable.reg spec ~width:64 in
  let%hw_var p2_c6 = Variable.reg spec ~width:64 in
  let%hw_var p2_c7 = Variable.reg spec ~width:64 in
  let%hw_var p2_c8 = Variable.reg spec ~width:64 in
  let%hw_var p2_c9 = Variable.reg spec ~width:64 in
  let%hw_var p2_cA = Variable.reg spec ~width:64 in
  let%hw_var p2_cB = Variable.reg spec ~width:64 in
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
                ; p2_sum <-- zero 64
                ; p2_r0 <-- zero 64
                ; p2_r1 <-- zero 64
                ; p2_r2 <-- zero 64
                ; p2_r3 <-- zero 64
                ; p2_r4 <-- zero 64
                ; p2_r5 <-- zero 64
                ; p2_r6 <-- zero 64
                ; p2_r7 <-- zero 64
                ; p2_r8 <-- zero 64
                ; p2_r9 <-- zero 64
                ; p2_rA <-- zero 64
                ; p2_rB <-- zero 64
                ; p2_c0 <-- zero 64
                ; p2_c1 <-- zero 64
                ; p2_c2 <-- zero 64
                ; p2_c3 <-- zero 64
                ; p2_c4 <-- zero 64
                ; p2_c5 <-- zero 64
                ; p2_c6 <-- zero 64
                ; p2_c7 <-- zero 64
                ; p2_c8 <-- zero 64
                ; p2_c9 <-- zero 64
                ; p2_cA <-- zero 64
                ; p2_cB <-- zero 64
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
                ; p2_c0 <-- push_digit p2_r1.value digit.value
                ; p2_c1 <-- push_digit p2_r2.value digit.value
                ; p2_c2 <-- push_digit p2_r3.value digit.value
                ; p2_c3 <-- push_digit p2_r4.value digit.value
                ; p2_c4 <-- push_digit p2_r5.value digit.value
                ; p2_c5 <-- push_digit p2_r6.value digit.value
                ; p2_c6 <-- push_digit p2_r7.value digit.value
                ; p2_c7 <-- push_digit p2_r8.value digit.value
                ; p2_c8 <-- push_digit p2_r9.value digit.value
                ; p2_c9 <-- push_digit p2_rA.value digit.value
                ; p2_cA <-- push_digit p2_rB.value digit.value
                ; p2_cB <-- digit.value
                ; p2_r0 <-- (digit_mask.value &: maximum p2_c0.value p2_r0.value)
                ; p2_r1 <-- (digit_mask.value &: maximum p2_c1.value p2_r1.value)
                ; p2_r2 <-- (digit_mask.value &: maximum p2_c2.value p2_r2.value)
                ; p2_r3 <-- (digit_mask.value &: maximum p2_c3.value p2_r3.value)
                ; p2_r4 <-- (digit_mask.value &: maximum p2_c4.value p2_r4.value)
                ; p2_r5 <-- (digit_mask.value &: maximum p2_c5.value p2_r5.value)
                ; p2_r6 <-- (digit_mask.value &: maximum p2_c6.value p2_r6.value)
                ; p2_r7 <-- (digit_mask.value &: maximum p2_c7.value p2_r7.value)
                ; p2_r8 <-- (digit_mask.value &: maximum p2_c8.value p2_r8.value)
                ; p2_r9 <-- (digit_mask.value &: maximum p2_c9.value p2_r9.value)
                ; p2_rA <-- (digit_mask.value &: maximum p2_cA.value p2_rA.value)
                ; p2_rB <-- (digit_mask.value &: maximum p2_cB.value p2_rB.value)
                ]
            ; when_ finish [ sm.set_next Done ]
            ] )
        ; ( Done
          , [ part1 <-- p1_sum.value +: p1_r0.value
            ; part1_valid <-- vdd
            ; part2 <-- p2_sum.value +: p2_r0.value
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
