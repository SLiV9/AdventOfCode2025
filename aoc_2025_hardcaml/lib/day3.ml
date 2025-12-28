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

let is_digit (byte : t) : t = uresize (srl byte ~by:4) ~width:1

let push_byte (old_value : t) (next_value : t) (byte : t) : t =
  let digit_value = uresize (uresize byte ~width:4) ~width:64 in
  let candidate = push_digit next_value digit_value in
  let new_value = maximum candidate old_value in
  mux2 (is_digit byte) new_value (zero 64)
;;

let create scope ({ clock; clear; start; finish; data_in; data_in_valid } : _ I.t) : _ O.t
  =
  let spec = Reg_spec.create ~clock ~clear () in
  let open Always in
  let sm = State_machine.create (module States) spec in
  let%hw_var p1_sum = Variable.reg spec ~width:64 in
  let%hw_var p1_r0 = Variable.reg spec ~width:64 in
  let%hw_var p1_r1 = Variable.reg spec ~width:64 in
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
  let part1 = Variable.wire ~default:(zero 64) () in
  let part1_valid = Variable.wire ~default:gnd () in
  let part2 = Variable.wire ~default:(zero 64) () in
  let part2_valid = Variable.wire ~default:gnd () in
  compile
    [ sm.switch
        [ ( Idle
          , [ when_
                start
                [ p1_sum <-- zero 64
                ; p1_r0 <-- zero 64
                ; p1_r1 <-- zero 64
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
                ; sm.set_next Processing
                ]
            ] )
        ; ( Processing
          , [ when_
                data_in_valid
                [ p1_sum <-- p1_sum.value +: mux2 (is_digit data_in) (zero 64) p1_r0.value
                ; p2_sum <-- p2_sum.value +: mux2 (is_digit data_in) (zero 64) p2_r0.value
                ; p1_r0 <-- push_byte p1_r0.value p1_r1.value data_in
                ; p1_r1 <-- push_byte p1_r1.value (zero 64) data_in
                ; p2_r0 <-- push_byte p2_r0.value p2_r1.value data_in
                ; p2_r1 <-- push_byte p2_r1.value p2_r2.value data_in
                ; p2_r2 <-- push_byte p2_r2.value p2_r3.value data_in
                ; p2_r3 <-- push_byte p2_r3.value p2_r4.value data_in
                ; p2_r4 <-- push_byte p2_r4.value p2_r5.value data_in
                ; p2_r5 <-- push_byte p2_r5.value p2_r6.value data_in
                ; p2_r6 <-- push_byte p2_r6.value p2_r7.value data_in
                ; p2_r7 <-- push_byte p2_r7.value p2_r8.value data_in
                ; p2_r8 <-- push_byte p2_r8.value p2_r9.value data_in
                ; p2_r9 <-- push_byte p2_r9.value p2_rA.value data_in
                ; p2_rA <-- push_byte p2_rA.value p2_rB.value data_in
                ; p2_rB <-- push_byte p2_rB.value (zero 64) data_in
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
