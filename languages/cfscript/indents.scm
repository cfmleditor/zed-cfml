; Blocks
(statement_block) @indent
("}" @end) @outdent

; Arrays and objects
(array) @indent
(object) @indent
("]" @end) @outdent

; Parentheses
(arguments) @indent
(formal_parameters) @indent
(parenthesized_expression) @indent
(")" @end) @outdent

; Switch cases
(switch_case) @indent
(switch_default) @indent
