; Blocks
(statement_block) @indent
("}" @end) @outdent

; Arrays and objects
(array) @indent
(object) @indent
("]" @end) @outdent

; Parentheses (multi-line)
(formal_parameters) @indent
(arguments) @indent
(parenthesized_expression) @indent

; Switch cases
(switch_case) @indent
(switch_default) @indent
