; CF tags
(erroneous_cf_end_tag_name) @tag
(cf_attribute_name) @attribute
(attribute_value) @string
(cf_selfclose_tag) @tag
(cf_start_tag_with_selfclose) @tag
(cf_set_tag) @tag
(cf_if_tag) @tag
(cf_else_tag) @tag
(cf_elseif_tag) @tag
(cf_start_tag) @tag
(cf_end_tag) @tag
(cf_tag) @tag
(cf_output_tag) @tag
(cf_return_tag) @tag
(cf_tag_name) @tag

; Variables
(identifier) @variable

; Properties
(property_identifier) @property
(shorthand_property_identifier) @property
(shorthand_property_identifier_pattern) @property
(private_property_identifier) @property

; Function and method definitions
(function_expression
  name: (identifier) @function)

(function_declaration
  name: (identifier) @function)

(function_declaration
  (access_type) @keyword)

(method_definition
  name: [
    (property_identifier)
    (private_property_identifier)
  ] @function)

(method_definition
  name: (property_identifier) @constructor
  (#eq? @constructor "constructor"))

(pair
  key: (property_identifier) @function
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (member_expression
    property: (property_identifier) @function)
  right: [(function_expression) (arrow_function)])

(variable_declarator
  name: (identifier) @function
  value: [(function_expression) (arrow_function)])

(assignment_expression
  left: (identifier) @function
  right: [(function_expression) (arrow_function)])

; Function and method calls
(call_expression
  function: (identifier) @function)

(call_expression
  function: (member_expression
    property: [
      (property_identifier)
      (private_property_identifier)
    ] @function))

; Literals
((identifier) @variable.special
  (#eq? @variable.special "self"))

(this) @variable.special
(super) @variable.special

(cf_var) @keyword

[
  (true)
  (false)
] @boolean

[
  (null)
  (undefined)
] @constant.builtin

[
  (comment)
  (cf_comment)
] @comment @spell

((comment) @comment.doc
  (#match? @comment.doc "^/[*][*][^*].*[*]/$"))

(string) @string
(hash_empty) @punctuation.special

(regex_pattern) @string.regex
(regex_flags) @string.special

(regex
  "/" @punctuation.bracket)

(number) @number

(hash_expression
  "#" @punctuation.special)

(unary_operator) @operator

(spread_element
  "..." @operator)

((identifier) @number
  (#any-of? @number "NaN" "Infinity"))

; Punctuation
[
  ";"
  "."
  ","
  ":"
  (optional_chain)
  (static_chain)
] @punctuation.delimiter

(binary_expression
  "/" @operator)

(ternary_expression
  [
    "?"
    ":"
  ] @keyword)

(elvis_expression
  "?:" @keyword)

[
  "-"
  "--"
  "-="
  "+"
  "++"
  "+="
  "*"
  "*="
  "**"
  "**="
  "/"
  "/="
  "%"
  "%="
  "<"
  "<="
  "<>"
  "<<"
  "<<="
  "="
  "=="
  "==="
  "!"
  "!="
  "!=="
  "=>"
  ">"
  ">="
  ">>"
  ">>="
  ">>>"
  ">>>="
  "~"
  "^"
  "&"
  "|"
  "^="
  "&="
  "|="
  "&&"
  (logical_or)
  "||"
  "??"
  "&&="
  "||="
  "??="
] @operator

[
  "var"
  "let"
  "const"
  "function"
  "new"
  "return"
  "if"
  "else"
  "for"
  "while"
  "do"
  "switch"
  "case"
  "default"
  "break"
  "continue"
  "try"
  "catch"
  "finally"
  "throw"
  "in"
  "of"
  "instanceof"
  "static"
  "with"
] @keyword

[
  "("
  ")"
  "["
  "]"
  "{"
  "}"
  "</"
] @punctuation.bracket

; SQL keywords and identifiers
(query_keyword) @keyword

(query_function
  name: (query_function_name) @function)

(query_function_name) @variable

(query_identifier) @variable
(query_alias
  right: _ @variable)

(query_star
  (star) @operator)

(query_number
  (number) @number)

(query_math_expression
  operator: _ @operator)

(query_comparison_expression
  operator: _ @operator)

; Quoted values: capture the whole node so the delimiters are styled too
[
  (quoted_query_value)
  (double_quoted_query_value)
] @string

; Backticks and brackets quote an identifier, not a string literal
(backtick_quoted_query_value
  (query_value) @variable)
(backtick_quoted_query_value
  "`" @punctuation.bracket)

(query_comma) @punctuation.delimiter
(query_semicolon) @punctuation.delimiter

(bracketed_query_value
  (query_value) @variable)
(bracketed_query_value
  ["[" "]"] @punctuation.bracket)

(parenthesized_query_node
  ["(" ")"] @punctuation.bracket)

(query_assignment_expression
  "=" @operator)

(query_operator) @operator

(query_open_paren) @punctuation.bracket
(query_close_paren) @punctuation.bracket
