letter ::= "A" | "B" | <...> | "Z" | "a" | "b" | <...> | "z"
digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" |"9"
octal_digit ::= "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7"
hex_digit ::= digit | "A"| "B" | "C" | "D" | "E" | "F"
identifier ::= (letter | ("_" (letter | digit))) {["_"] (letter | digit)}
constant ::= numeric_literal | character_string | time_literal | bit_string_literal |
boolean_literal
numeric_literal ::= integer_literal | real_literal
integer_literal ::= [ integer_type_name "#" ]( signed_integer | binary_integer |
octal_integer | hex_integer)
signed_integer ::= ["+" |"-"] integer
integer ::= digit {["_"] digit}
binary_integer ::= "2#" bit {["_"] bit}
bit ::= "1" | "0"
octal_integer ::= "8#" octal_digit {["_"] octal_digit}
hex_integer ::= "16#" hex_digit{["_"] hex_digit}
real_literal ::= [ real_type_name "#" ]
signed_integer "." integer [exponent]
exponent ::= ("E" | "e")["+"|"-"] integer
bit_string_literal ::= [ ("BYTE" | "WORD" | "DWORD" | "LWORD") "#" ]( unsigned_integer |
binary_integer | octal_integer | hex_integer)
boolean_literal ::= ( [ "BOOL#" ] ("1" | "0" ) )| "TRUE" | "FALSE"
character_string ::= single_byte_character_string | double_byte_character_string
single_byte_character_string ::= """ {single_byte_character_representation} """
double_byte_character_string ::= """ {double_byte_character_representation} """
single_byte_character_representation ::= common_character_representation | "$"" | """ |
"$" hex_digit hex_digit
double_byte_character_representation ::= common_character_representation | "$"" | """|
"$" hex_digit hex_digit hex_digit hex_digit
common_character_representation ::= <any printable character except "$", """ or """> |
"$$" | "$L" | "$N" | "$P" | "$R" | "$T" | "$l" | "$n" | "$p" | "$r" | "$t"
time_literal ::= duration | time_of_day
duration ::= ("T" | "TIME") "#" ["-"] interval
interval ::= days | hours | minutes | seconds | milliseconds
days ::= fixed_point ("d") | integer ("d") ["_"] hours
fixed_point ::= integer [ "." integer]
hours ::= fixed_point ("h") | integer ("h")["_"] minutes
minutes ::= fixed_point ("m") | integer ("m") ["_"] seconds
seconds ::= fixed_point ("s") | integer ("s") ["_"]milliseconds
milliseconds ::= fixed_point ("ms")
time_of_day ::= ("TIME_OF_DAY" | "TOD") "#" daytime
daytime ::= day_hour ":" day_minute ":" day_second
day_hour ::= integer
day_minute ::= integer
day_second ::= fixed_point
date ::= ("DATE" | "D") "#" date_literal
date_literal ::= year "-" month "-" day
year ::= integer
month ::= integer
day ::= integer
date_and_time ::= ("DATE_AND_TIME" | "DT") "#" date_literal "-" daytime
data_type_name ::= non_generic_type_name | generic_type_name
non_generic_type_name ::= elementary_type_name | derived_type_name
elementary_type_name ::= numeric_type_name | date_type_name | bit_string_type_name
| "STRING" | "WSTRING" | "TIME"
numeric_type_name ::= integer_type_name | real_type_name
integer_type_name ::= signed_integer_type_name | unsigned_integer_type_name
signed_integer_type_name ::= "SINT" | "INT" | "DINT" | "LINT"
unsigned_integer_type_name ::= "USINT" | "UINT" | "UDINT" | "ULINT"
real_type_name ::= "REAL" | "LREAL"
date_type_name ::= "DATE" | "TIME_OF_DAY" | "TOD" | "DATE_AND_TIME" | "DT"
bit_string_type_name ::= "BOOL" | "BYTE" | "WORD" | "DWORD" | "LWORD"
generic_type_name ::= "ANY" | "ANY_DERIVED" | "ANY_ELEMENTARY" | "ANY_MAGNITUDE"
| "ANY_NUM" | "ANY_REAL" | "ANY_INT" | "ANY_BIT" | "ANY_STRING" | "ANY_DATE"
derived_type_name ::= single_element_type_name | array_type_name | structure_type_name |
string_type_name
single_element_type_name ::= simple_type_name | subrange_type_name | enumerated_type_name
simple_type_name ::= identifier
subrange_type_name ::= identifier
enumerated_type_name ::= identifier
array_type_name ::= identifier
structure_type_name ::= identifier
data_type_declaration ::= "TYPE" type_declaration ";" {type_declaration ";"} "END_TYPE"
type_declaration ::= single_element_type_declaration | array_type_declaration |
structure_type_declaration | string_type_declaration
single_element_type_declaration ::= simple_type_declaration | subrange_type_declaration |
enumerated_type_declaration
simple_type_declaration ::= simple_type_name ":" simple_spec_init
simple_spec_init ::= simple_specification [":=" constant]
simple_specification ::= elementary_type_name | simple_type_name
subrange_type_declaration ::= subrange_type_name ":" subrange_spec_init
subrange_spec_init ::= subrange_specification[":=" signed_integer]
subrange_specification ::= integer_type_name "(" subrange")" | subrange_type_name
subrange ::= signed_integer ".." signed_integer
enumerated_type_declaration ::= enumerated_type_name ":" numerated_spec_init
enumerated_spec_init ::= enumerated_specification [":=" numerated_value]
enumerated_specification ::= ("(" enumerated_value {"," enumerated_value} ")") |
enumerated_type_name
enumerated_value ::= [enumerated_type_name "#"] identifier
array_type_declaration ::= array_type_name ":" array_spec_init
array_spec_init ::= array_specification [":=" array_initialization]
array_specification ::= array_type_name | "ARRAY" "[" subrange {"," subrange} "]"
"OF" non_generic_type_name
array_initialization::= "[" array_initial_elements {"," array_initial_elements} "]"
array_initial_elements ::= array_initial_element | integer "("[array_initial_element] ")"
array_initial_element ::= constant | enumerated_value | structure_initialization |
array_initialization
structure_type_declaration ::= structure_type_name ":" structure_specification
structure_specification ::= structure_declaration | initialized_structure
initialized_structure ::=structure_type_name [":=" structure_initialization]
structure_declaration ::="STRUCT" structure_element_declaration ";
"{structure_element_declaration ";"}"END_STRUCT"
structure_element_declaration ::= structure_element_name ":"(simple_spec_init |
subrange_spec_init | enumerated_spec_init | array_spec_init | initialized_structure)
structure_element_name ::= identifier
structure_initialization ::= "(" structure_element_initialization
{"," structure_element_initialization} ")"
structure_element_initialization ::= structure_element_name ":=" (constant | enumerated_value
| array_initialization | structure_initialization)
string_type_name ::= identifier
string_type_declaration ::= string_type_name ":" ("STRING"|"WSTRING") ["[" integer "]"]
[":=" character_string]
variable ::= direct_variable | symbolic_variable
symbolic_variable ::= variable_name | multi_element_variable
variable_name ::= identifier
direct_variable ::= "%" location_prefix size_prefix integer {"." integer}
location_prefix ::= "I" | "Q" | "M"
size_prefix ::= NIL | "X" | "B" | "W" | "D" | "L"
multi_element_variable ::= array_variable | structured_variable
array_variable ::= subscripted_variable subscript_list
subscripted_variable ::= symbolic_variable
subscript_list ::= "[" subscript {"," subscript} "]"
subscript ::= expression
structured_variable ::= record_variable "." field_selector
record_variable ::= symbolic_variable
field_selector ::= identifier
input_declarations ::= "VAR_INPUT" ["RETAIN" | "NON_RETAIN"] input_declaration ";"
{input_declaration ";"} "END_VAR"
input_declaration ::= var_init_decl | edge_declaration
edge_declaration ::= var1_list ":" "BOOL" ("R_EDGE" | "F_EDGE")
var_init_decl ::= var1_init_decl | array_var_init_decl | structured_var_init_decl
| fb_name_decl | string_var_declaration
var1_init_decl ::= var1_list ":" (simple_spec_init | subrange_spec_init |
enumerated_spec_init)
var1_list ::= variable_name {"," variable_name}
array_var_init_decl ::= var1_list ":" array_spec_init
structured_var_init_decl ::= var1_list ":" initialized_structure
fb_name_decl ::= fb_name_list ":" function_block_type_name [ ":=" structure_initialization ]
fb_name_list ::= fb_name {"," fb_name}
fb_name ::= identifier
output_declarations ::= "VAR_OUTPUT" ["RETAIN" | "NON_RETAIN"] var_init_decl ";"
{var_init_decl ";"} "END_VAR"
input_output_declarations ::= "VAR_IN_OUT" var_declaration ";" {var_declaration ";"} "END_VAR"
var_declaration ::= temp_var_decl | fb_name_decl
temp_var_decl ::= var1_declaration | array_var_declaration | structured_var_declaration |
string_var_declaration
var1_declaration ::= var1_list ":" (simple_specification | subrange_specification |
enumerated_specification)
array_var_declaration ::= var1_list ":" array_specification
structured_var_declaration ::= var1_list ":" structure_type_name
var_declarations ::= "VAR" ["CONSTANT"] var_init_decl ";" {(var_init_decl ";")}
"END_VAR"
retentive_var_declarations ::= "VAR" "RETAIN" var_init_decl ";" {var_init_decl ";"}
"END_VAR"
located_var_declarations ::= "VAR" ["CONSTANT" | "RETAIN" | "NON_RETAIN"]
located_var_decl ";" {located_var_decl ";"} "END_VAR"
located_var_decl ::= [variable_name] location ":" located_var_spec_init
external_var_declarations ::= "VAR_EXTERNAL" ["CONSTANT"]
external_declaration ";" {external_declaration ";"} "END_VAR"
external_declaration ::= global_var_name ":" (simple_specification |
subrange_specification | enumerated_specification | array_specification |
structure_type_name | function_block_type_name)
global_var_name ::= identifier
global_var_declarations ::= "VAR_GLOBAL" ["CONSTANT" | "RETAIN"]
global_var_decl ";" {global_var_decl ";"} "END_VAR"
global_var_decl ::= global_var_spec ":"
[ located_var_spec_init | function_block_type_name ]
global_var_spec ::= global_var_list | [global_var_name] location
located_var_spec_init ::= simple_spec_init | subrange_spec_init
| enumerated_spec_init | array_spec_init | initialized_structure |
single_byte_string_spec | double_byte_string_spec
location ::= "AT" direct_variable
global_var_list ::= global_var_name {"," global_var_name}
string_var_declaration ::= single_byte_string_var_declaration
| double_byte_string_var_declaration
single_byte_string_var_declaration ::= var1_list ":" single_byte_string_spec
Advances in Computational Intelligence, Man-Machine Systems and Cybernetics
ISBN: 978-960-474-257-8
174
single_byte_string_spec ::= "STRING" ["[" integer "]"] [":=" single_byte_character_string]
double_byte_string_var_declaration ::= var1_list ":" double_byte_string_spec
double_byte_string_spec ::= "WSTRING" ["[" integer "]"] [":=" double_byte_character_string]
incompl_located_var_declarations ::= "VAR" ["RETAIN"|"NON_RETAIN"]
incompl_located_var_decl ";" {incompl_located_var_decl ";"} "END_VAR"
incompl_located_var_decl ::= variable_name incompl_location ":" var_spec
incompl_location ::= "AT" "%" ("I" | "Q" | "M") "*"
var_spec ::= simple_specification | subrange_specification | enumerated_specification |
array_specification | structure_type_name | "STRING" ["[" integer "]"] | "WSTRING" ["["integer "]"]
function_name ::= standard_function_name | derived_function_name
standard_function_name ::= <as defined in clause 2.5.1.5 of the standard>
derived_function_name ::= identifier
function_declaration ::= "FUNCTION" derived_function_name ":"
(elementary_type_name | derived_type_name)
{ io_var_declarations | function_var_decls } function_body
"END_FUNCTION"
io_var_declarations ::= input_declarations | output_declarations |
input_output_declarations
function_var_decls ::= "VAR" ["CONSTANT"]
var2_init_decl ";" {var2_init_decl ";"} "END_VAR"
function_body ::= ladder_diagram | function_block_diagram |
instruction_list | statement_list | <other languages>
var2_init_decl ::= var1_init_decl | array_var_init_decl |
structured_var_init_decl | string_var_declaration
This syntax do not include the fact each function must
have at least one input declaration. Also, It does not include
either the fact that declarations of edge, references and in-
vocations to function blocks are not allowed in the body of
functions according to the standard [2].
function_block_type_name ::= standard_function_block_name
| derived_function_block_name
standard_function_block_name ::= <as defined in clause 2.5.2.3 of the
standard>
derived_function_block_name ::= identifier
function_block_declaration ::=
"FUNCTION_BLOCK" derived_function_block_name
{ io_var_declarations | other_var_declarations }
function_block_body
"END_FUNCTION_BLOCK"
other_var_declarations ::= external_var_declarations | var_declarations
| retentive_var_declarations | non_retentive_var_declarations
| temp_var_decls | incompl_located_var_declarations
temp_var_decls ::=
"VAR_TEMP"
temp_var_decl ";"
{temp_var_decl ";"}
"END_VAR"
non_retentive_var_decls ::=
"VAR" "NON_RETAIN"
var_init_decl ";"
{var_init_decl ";"}
"END_VAR"
function_block_body ::= ladder_diagram | function_block_diagram
| instruction_list | statement_list | <other languages>
program_type_name :: = identifier
program_declaration ::=
"PROGRAM" program_type_name
{ io_var_declarations | other_var_declarations
| located_var_declarations | program_access_decls }
function_block_body
"END_PROGRAM"
program_access_decls ::=
"VAR_ACCESS" program_access_decl ";"
{program_access_decl ";" }
"END_VAR"
program_access_decl ::= access_name ":" symbolic_variable ":"
non_generic_type_name [direction]
configuration_name ::= identifier
resource_type_name ::= identifier
configuration_declaration ::=
"CONFIGURATION" configuration_name
[global_var_declarations]
(single_resource_declaration
| (resource_declaration {resource_declaration}))
[access_declarations]
[instance_specific_initializations]
"END_CONFIGURATION"
resource_declaration ::=
"RESOURCE" resource_name "ON" resource_type_name
[global_var_declarations]
single_resource_declaration
"END_RESOURCE"
single_resource_declaration ::=
{task_configuration ";"}
program_configuration ";"
{program_configuration ";"}
resource_name ::= identifier
access_declarations ::=
"VAR_ACCESS"
access_declaration ";"
{access_declaration ";"}
"END_VAR"
access_declaration ::= access_name ":" access_path ":"
non_generic_type_name [direction]
access_path ::= [resource_name "."] direct_variable
| [resource_name "."] [program_name "."]
{fb_name"."} symbolic_variable
global_var_reference ::=
[resource_name "."] global_var_name ["." structure_element_name]
access_name ::= identifier
program_output_reference ::= program_name "." symbolic_variable
program_name ::= identifier
direction ::= "READ_WRITE" | "READ_ONLY"
task_configuration ::= "TASK" task_name task_initialization
task_name := identifier
task_initialization ::=
"(" ["SINGLE" ":=" data_source ","]
["INTERVAL" ":=" data_source ","]
"PRIORITY" ":=" integer ")"
data_source ::= constant | global_var_reference
| program_output_reference | direct_variable
program_configuration ::=
"PROGRAM" [RETAIN | NON_RETAIN]
program_name ["WITH" task_name] ":" program_type_name
["(" prog_conf_elements ")"]
prog_conf_elements ::= prog_conf_element {"," prog_conf_element}
prog_conf_element ::= fb_task | prog_cnxn
fb_task ::= fb_name "WITH" task_name
prog_cnxn ::= symbolic_variable ":=" prog_data_source
| symbolic_variable "=>" data_sink
prog_data_source ::= constant | enumerated_value | global_var_reference |
direct_variable
data_sink ::= global_var_reference | direct_variable
instance_specific_initializations ::=
"VAR_CONFIG"
instance_specific_init ";"
{instance_specific_init ";"}
"END_VAR"
instance_specific_init ::=
resource_name "." program_name "." {fb_name "."}
((variable_name [location] ":" located_var_spec_init) |
(fb_name ":" function_block_type_name ":="
structure_initialization))
3.2 Production rules for the elements of the
IL programming language
In this section, the production rules that describe the el-
ements of the IL programming language are presented,
which correspond to instructions,operands and operators.
instruction_list ::= il_instruction {il_instruction}
il_instruction ::= [label":"] [ il_simple_operation
| il_expression
| il_jump_operation
| il_fb_call
| il_formal_funct_call
| il_return_operator ] EOL {EOL}
label ::= identifier
il_simple_operation ::= ( il_simple_operator [il_operand] )
| ( function_name [il_operand_list] )
il_expression ::= il_expr_operator "(" [il_operand] EOL {EOL}
[simple_instr_list] ")"
il_jump_operation ::= il_jump_operator label
il_fb_call ::= il_call_operator fb_name ["("
(EOL {EOL} [ il_param_list ]) | [ il_operand_list ] ")"]
il_formal_funct_call ::= function_name "(" EOL {EOL} [il_param_list] ")"
il_operand ::= constant | variable | enumerated_value
il_operand_list ::= il_operand {"," il_operand}
simple_instr_list ::= il_simple_instruction {il_simple_instruction}
il_simple_instruction ::=
(il_simple_operation | il_expression | il_formal_funct_call)
EOL {EOL}
il_param_list ::= {il_param_instruction} il_param_last_instruction
il_param_instruction ::= (il_param_assignment | il_param_out_assignment)
"," EOL {EOL}
il_param_last_instruction ::=
( il_param_assignment | il_param_out_assignment ) EOL {EOL}
il_param_assignment ::= il_assign_operator ( il_operand | ( "(" EOL {EOL}
simple_instr_list ")" ) )
il_param_out_assignment ::= il_assign_out_operator variable
il_simple_operator ::= "LD" | "LDN" | "ST" | "STN" | "NOT" | "S"
| "R" | "S1" | "R1" | "CLK" | "CU" | "CD" | "PV"
| "IN" | "PT" | il_expr_operator
il_expr_operator ::= "AND" | "&" | "OR" | "XOR" | "ANDN" | "\&N" | "ORN"
| "XORN" | "ADD" | "SUB" | "MUL" | "DIV" | "MOD" | "GT" | "GE" | "EQ"
| "LT" | "LE" | "NE"
il_assign_operator ::= variable_name":="
il_assign_out_operator ::= ["NOT"] variable_name"=>"
il_call_operator ::= "CAL" | "CALC" | "CALCN"
il_return_operator ::= "RET" | "RETC" | "RETCN"
il_jump_operator ::= "JMP" | "JMPC" | "JMPCN"
3.3 Production rules for the elements of the
ST programming language
In this section, the production rules that describe the el-
ements of the ST programming language are presented,
which correspond to expressions and sentences.
expression ::= xor_expression {"OR" xor_expression}
xor_expression ::= and_expression {"XOR" and_expression}
and_expression ::= comparison {("&" | "AND") comparison}
comparison ::= equ_expression { ("=" | "<>") equ_expression}
equ_expression ::= add_expression {comparison_operator add_expression}
comparison_operator ::= "<" | ">" | "<=" | ">=" "
add_expression ::= term {add_operator term}
Advances in Computational Intelligence, Man-Machine Systems and Cybernetics
ISBN: 978-960-474-257-8
175
add_operator ::= "+" | "-"
term ::= power_expression {multiply_operator power_expression}
multiply_operator ::= "*" | "/" | "MOD"
power_expression ::= unary_expression {"**" unary_expression}
unary_expression ::= [unary_operator] primary_expression
unary_operator ::= "-" | "NOT"
primary_expression ::=
constant | enumerated_value | variable | "(" expression ")"
| function_name "(" param_assignment {"," param_assignment} ")"
statement_list ::= statement ";" {statement ";"}
statement ::= NIL | assignment_statement |subprogram_control_statement
| selection_statement | iteration_statement
assignment_statement ::= variable ":=" expression
subprogram_control_statement ::= fb_invocation | "RETURN"
fb_invocation ::= fb_name "(" [param_assignment {"," param_assignment}]")"
param_assignment ::= ([variable_name ":="] expression)
| (["NOT"] variable_name "=>" variable)
selection_statement ::= if_statement | case_statement
if_statement ::=
"IF" expression "THEN" statement_list
{"ELSIF" expression "THEN" statement_list}
["ELSE" statement_list]
"END_IF"
case_statement ::=
"CASE" expression "OF"
case_element
{case_element}
["ELSE" statement_list]
"END_CASE"
case_element ::= case_list ":" statement_list
case_list ::= case_list_element {"," case_list_element}
case_list_element ::= subrange | signed_integer | enumerated_value
iteration_statement ::=
for_statement | while_statement | repeat_statement | exit_statement
for_statement ::=
"FOR" control_variable ":=" for_list "DO" statement_list "END_FOR"
control_variable ::= identifier
for_list ::= expression "TO" expression ["BY" expression]
while_statement ::= "WHILE" expression "DO" statement_list "END_WHILE"
repeat_statement ::=
"REPEAT" statement_list "UNTIL" expression "END_REPEAT"
exit_statement ::= "EXIT"