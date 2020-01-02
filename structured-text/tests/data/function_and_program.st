PROGRAM stexample
  VAR
    x : BOOL;
  END_VAR
  x := TRUE;
  REPEAT
    x := FALSE;
  UNTIL x := FALSE;
  END_REPEAT;
END_PROGRAM;

FUNCTION add: INT
    VAR_INPUT
        a: INT;
        b: INT;
    END_VAR
    VAR_OUTPUT
        result: INT;
    END_VAR

    result := a + b;
END_FUNCTION;
