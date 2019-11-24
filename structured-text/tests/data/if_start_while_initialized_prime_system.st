// https://owlcation.com/stem/Introduction-to-Structured-Text-With-Codesys-35#mod_46205880
IF Start THEN
    Start:=FALSE; //Remove Start Latch
    WHILE Initialised = FALSE DO
        System_Prime:=TRUE; //Set Output On For System Primer
    END_WHILE
    System_Prime:=FALSE; //Turn Off Output For System Primer Once Initialised
END_IF