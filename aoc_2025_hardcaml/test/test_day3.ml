open! Core
open! Hardcaml
open! Hardcaml_waveterm
open! Hardcaml_test_harness
module Day3 = Aoc_2025_hardcaml.Day3
module Harness = Cyclesim_harness.Make (Day3.I) (Day3.O)

let ( <--. ) = Bits.( <--. )
let sample_input_values = [ 52; 51; 52; 49; 10 ]

let simple_testbench (sim : Harness.Sim.t) =
  let inputs = Cyclesim.inputs sim in
  let outputs = Cyclesim.outputs sim in
  let cycle ?n () = Cyclesim.cycle ?n sim in
  (* Helper function for inputting one value *)
  let feed_input n =
    inputs.data_in <--. n;
    inputs.data_in_valid := Bits.vdd;
    cycle ();
    inputs.data_in_valid := Bits.gnd;
    cycle ()
  in
  (* Reset the design *)
  inputs.clear := Bits.vdd;
  cycle ();
  inputs.clear := Bits.gnd;
  cycle ();
  (* Pulse the start signal *)
  inputs.start := Bits.vdd;
  cycle ();
  inputs.start := Bits.gnd;
  (* Input some data *)
  List.iter sample_input_values ~f:(fun x -> feed_input x);
  inputs.finish := Bits.vdd;
  cycle ();
  inputs.finish := Bits.gnd;
  cycle ();
  (* Wait for result to become valid *)
  while not (Bits.to_bool !(outputs.part1.valid)) do
    cycle ()
  done;
  let part1 = Bits.to_unsigned_int !(outputs.part1.value) in
  let part2 = Bits.to_unsigned_int !(outputs.part2.value) in
  print_s [%message "Result" (part1 : int) (part2 : int)];
  (* Show in the waveform that [valid] stays high. *)
  cycle ~n:2 ()
;;

let waves_config =
  Waves_config.to_directory "/tmp/"
  |> Waves_config.as_wavefile_format ~format:Hardcamlwaveform
;;

let%expect_test "happy" =
  Harness.run_advanced
    ~waves_config
    ~create:Day3.hierarchical
    ~trace:`All_named
    simple_testbench;
  [%expect
    {|
    (Result (part1 44) (part2 4341))
    Saved waves to /tmp/test_day3_ml_happy.hardcamlwaveform
    |}]
;;
