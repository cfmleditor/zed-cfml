; Function definitions
(function_declaration
  name: (identifier) @name) @item

(function_expression
  name: (identifier) @name) @item

(method_definition
  name: (property_identifier) @name) @item

; cffunction tag
(cf_tag
  (cf_start_tag
    (cf_tag_name) @_cffunction
    (cf_tag_attributes
      (cf_attribute
        (cf_attribute_name) @_name
        (quoted_cf_attribute_value
          (attribute_value) @name))))
  (#eq? @_cffunction "function")
  (#eq? @_name "name")) @item
