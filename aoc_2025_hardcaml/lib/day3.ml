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

let create scope ({ clock; clear; start; finish; data_in; data_in_valid } : _ I.t) : _ O.t
  =
  let spec = Reg_spec.create ~clock ~clear () in
  let open Always in
  let sm = State_machine.create (module States) spec in
  let%hw_var p1_sum = Variable.reg spec ~width:64 in
  let%hw_var p1_row0 = Variable.reg spec ~width:64 in
  let%hw_var p1_row1 = Variable.reg spec ~width:64 in
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
                ; p1_row0 <-- zero 64
                ; p1_row1 <-- zero 64
                ; sm.set_next Processing
                ]
            ] )
        ; ( Processing
          , [ when_ data_in_valid [ p1_sum <-- p1_sum.value +: uresize data_in ~width:64 ]
            ; when_ finish [ sm.set_next Done ]
            ] )
        ; Done, [ part1_valid <-- vdd; when_ finish [ sm.set_next Processing ] ]
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
