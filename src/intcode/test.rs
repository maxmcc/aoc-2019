mod test_day02 {
    use crate::intcode::*;

    #[test]
    fn test_step() {
        let program = Program::from(&[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]);
        let mut machine = Machine::default_io(&program);
        machine.step();
        assert_eq!(
            machine.memory,
            [1, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].iter().into()
        );
        machine.step();
        assert_eq!(
            machine.memory,
            [3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50].iter().into()
        );
        let status = machine.step();
        assert_eq!(status, vm::Status::Halted);
    }

    #[test]
    fn test_begin_end_states() {
        let states: &[(Program, mem::Memory)] = &[
            (
                [1, 0, 0, 0, 99].iter().into(),
                [2, 0, 0, 0, 99].iter().into(),
            ),
            (
                [2, 3, 0, 3, 99].iter().into(),
                [2, 3, 0, 6, 99].iter().into(),
            ),
            (
                [2, 4, 4, 5, 99, 0].iter().into(),
                [2, 4, 4, 5, 99, 9801].iter().into(),
            ),
            (
                [1, 1, 1, 4, 99, 5, 6, 0, 99].iter().into(),
                [30, 1, 1, 4, 2, 5, 6, 0, 99].iter().into(),
            ),
        ];
        for (program, memory) in states {
            let mut machine = Machine::default_io(program);
            let status = machine.run();
            assert_eq!(status, vm::Status::Halted);
            assert_eq!(&machine.memory, memory);
        }
    }
}

mod test_day05 {
    use crate::intcode::*;

    fn machine_io_test<'a>(
        program: &Program,
        expected: impl IntoIterator<Item = &'a (isize, isize)>,
    ) {
        for (input, output) in expected.into_iter() {
            let mut machine = Machine::default_io(program);
            machine.input.queue.push_back(input.into());
            let status = machine.run();
            assert_eq!(status, vm::Status::Halted);
            assert_eq!(machine.output.buffer, [output.into()]);
        }
    }

    #[test]
    fn test_eq_8_position_mode() {
        let program = Program::from(&[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8]);
        let expected = &[(8, 1), (7, 0)];
        machine_io_test(&program, expected);
    }

    #[test]
    fn test_lt_8_position_mode() {
        let program = Program::from(&[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8]);
        let expected = &[(5, 1), (8, 0), (10, 0)];
        machine_io_test(&program, expected);
    }
    #[test]
    fn test_eq_8_immediate_mode() {
        let program = Program::from(&[3, 3, 1108, -1, 8, 3, 4, 3, 99]);
        let expected = &[(8, 1), (7, 0)];
        machine_io_test(&program, expected);
    }

    #[test]
    fn test_lt_8_immediate_mode() {
        let program = Program::from(&[3, 3, 1107, -1, 8, 3, 4, 3, 99]);
        let expected = &[(5, 1), (8, 0), (12, 0)];
        machine_io_test(&program, expected);
    }

    #[test]
    fn test_nz_position_mode() {
        let program = Program::from(&[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9]);
        let expected = &[(0, 0), (1, 1), (5, 1)];
        machine_io_test(&program, expected);
    }

    #[test]
    fn test_nz_immediate_mode() {
        let program = Program::from(&[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1]);
        let expected = &[(0, 0), (1, 1), (39, 1)];
        machine_io_test(&program, expected);
    }

    #[test]
    fn test_big_input() {
        let program = Program::from(
            [
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ]
            .iter(),
        );
        let expected = &[(7, 999), (8, 1000), (9, 1001)];
        machine_io_test(&program, expected);
    }
}
