// https://owlcation.com/stem/Introduction-to-Structured-Text-With-Codesys-35#mod_46205970
IF S1 = 0 THEN
    S1:= S1+1;
ELSIF S1 = 1 THEN
    S1:= 2;
ELSIF S1 = 2 THEN
    S1:= 3;
ELSE
    S1:=4;
END_IF