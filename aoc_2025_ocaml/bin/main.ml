let () = print_endline "Hello, World!"

type solver = { mutable sum: int; mutable row: int list }

let finalize (s: solver) : int =
  s.sum + s.row.hd

let process (s: solver) (x: int) : unit  =
  let is_digit = (x land 0b00010000) lsr 4;
  let digit_value = x land 0b00001111;
  let break_mask = (is_digit lsl 60) - 1;
  s.sum <- s.sum + (break_mask land s.row.hd);
  s.row <- process1(s.row, lnot break_mask);
  ()
