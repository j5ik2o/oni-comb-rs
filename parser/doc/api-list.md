# api list

## factory methods

### core

| parser | description |
| ------------- | ------------- |
| unit | Returns a Parser that does nothing. |
| empty | Returns a Parser that does nothing. It is an alias for `unit()`.  |
| end | Returns a Parser representing the termination. |
| successful | Returns a Parser representing the successful parsing result. |
| successful_lazy | Returns a Parser representing the successful parsing result. |
| failed | Returns a Parser that represents the result of the failed parsing. |
| failed_with_commit | Returns a Parser that returns and commits the failed parsing result. |
| failed_with_uncommit | Returns a Parser that returns failed parsing results and does not commit. |
| failed_lazy | Returns a Parser that represents the result of the failed parsing. |

### for element

| parser | description |
| ------------- | ------------- |
| elm_any_ref | Returns a Parser that parses an any element.(for reference) |
| elm_any | Returns a Parser that parses an any element. |
| elm_ref | Returns a Parser that parses the specified element.(for reference) |
| elm | Returns a Parser that parses the specified element. |
| elm_pred_ref | Returns a Parser that parses the elements that satisfy the specified closure conditions.(for reference) |
| elm_pred | Returns a Parser that parses the elements that satisfy the specified closure conditions. |
| elm_ref_of | Returns a Parser that parses the elements in the specified set. (for reference) |
| elm_of | Returns a Parser that parses the elements in the specified set. |
| elm_in_ref | Returns a Parser that parses the elements in the specified range. (for reference) |
| elm_in | Returns a Parser that parses the elements in the specified range. |
| elm_from_until_ref | Returns a Parser that parses the elements in the specified range. (for reference) |
| elm_from_until | Returns a Parser that parses the elements in the specified range. |
| none_ref_of | Returns a Parser that parses elements that do not contain elements of the specified set.(for reference) |
| none_of | Returns a Parser that parses elements that do not contain elements of the specified set. |
| elm_space_ref | Returns a Parser that parses the space (' ', '\t'). (for reference) |
| elm_space | Returns a Parser that parses the space (' ', '\t'). |
| elm_multi_space_ref | Returns a Parser that parses spaces containing newlines (' ', '\t', '\n', '\r'). (for reference) |
| elm_multi_space | Returns a Parser that parses spaces containing newlines (' ', '\t', '\n', '\r'). |
| elm_alpha_ref | Returns a Parser that parses alphabets ('A'..='Z', 'a'..='z').(for reference) |
| elm_alpha | Returns a Parser that parses alphabets ('A'..='Z', 'a'..='z'). |
| elm_alpha_digit_ref | Returns a Parser that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z').(for reference) |
| elm_alpha_digit | Returns a Parser that parses alphabets and digits ('0'..='9', 'A'..='Z', 'a'..='z'). |
| elm_digit_ref | Returns a Parser that parses digits ('0'..='9').(for reference) |
| elm_digit | Returns a Parser that parses digits ('0'..='9'). |
| elm_digit_1_9_ref | Returns a Parser that parses digits ('1'..='9').(for reference) |
| elm_digit_1_9 | Returns a Parser that parses digits ('1'..='9'). |
| elm_hex_digit_ref | Returns a Parser that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f').(for reference) |
| elm_hex_digit | Returns a Parser that parses hex digits ('0'..='9', 'A'..='F', 'a'..='f'). |
| elm_oct_digit_ref | Returns a Parser that parses oct digits ('0'..='8').(for reference) |
| elm_oct_digit | Returns a Parser that parses oct digits ('0'..='8'). |

### for elements

| parser | description |
| ------------- | ------------- |
| seq | Returns a Parser that parses a sequence of elements. |
| tag | Returns a Parser that parses a string. |
| tag_no_case | Returns a Parser that parses a string. However, it is not case-sensitive. |
| regex | Returns a Parser that parses a string that match a regular expression. |
| take | Returns a Parser that returns an element of the specified length. |
| take_while0 | Returns a Parser that returns elements, while the result of the closure is true. The length of the analysis result is not required. |
| take_while1 | Returns a Parser that returns elements, while the result of the closure is true. The length of the analysis result must be at least one element. |
| take_while_n_m | Returns a Parser that returns elements, while the result of the closure is true. The length of the analysis result should be between n and m elements. |
| take_till0 |  Returns a Parser that returns a sequence up to either the end element or the element that matches the condition. The length of the analysis result must be at least one element. |
| take_till1 | Returns a Parser that returns a sequence up to either the end element or the element that matches the condition. The length of the analysis result must be at least one element. |

### misc

| parser | description |
| ------------- | ------------- |
| skip | Returns a Parser that skips the specified number of elements. |
| surround | Returns a parser that parses three enumrated parsers and then discards the parsed results of the previous and next parsers. |
| lazy | Returns a parser that delays the initialization and evaluation of the parser passed as argument. |

## combinators

### parse

| combinator | description |
| ------------- | ------------- |
| parse | |
| run | |

### core

| combinator | description |
| ------------- | ------------- |
| flat_map | |
| map | |
| filter | |

### logging

| combinator | description |
| ------------- | ------------- |
| log | |
| debug | |
| info | |
| warn | |
| error | |
| name | |

### offset

| combinator | description |
| ------------- | ------------- |
| last_offset | |
| next_offset | |

### binary operator

| combinator | description |
| ------------- | ------------- |
| and_then | |
| or | |

### unary operator

| not | |
| opt | |


### chain operator

| scan_right1 | |
| chain_right0 | |
| chain_left0 | |
| chain_right1 | |
| chain_left1 | |
| rest_right1 | |
| rest_left1 | |

### skip

| combinator | description |
| ------------- | ------------- |
| skip_left | |
| skip_right | |
| surround | |

### repeat

| combinator | description |
| ------------- | ------------- |
| repeat | |
| of_many0 | |
| of_many1 | |
| of_many_n_m | |
| of_count | |
| of_rep_sep | |
| of_many0_sep | |
| of_many1_sep | |
| of_many_n_m_sep | |
| of_count_sep | |

### extension

| combinator | description |
| ------------- | ------------- |
| exists | |
| cache | |
| collect | |
| map_res | |
| discard | |
| attempt | |
| peek | |



