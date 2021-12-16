# api list

## factory methods

### base

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

### basic

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

## combinators
