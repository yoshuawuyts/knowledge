# tachyons

## Type scale
```css
/*

   TYPE SCALE

*/
/* For Hero Titles */
.f-6, .f-headline { font-size: 6rem; }
.f-5, .f-subheadline { font-size: 5rem; }
/* Type Scale */
.f1 { font-size: 3rem; }
.f2 { font-size: 2.25rem; }
.f3 { font-size: 1.5rem; }
.f4 { font-size: 1.25rem; }
.f5 { font-size: 1rem; }
.f6 { font-size: .875rem; }
@media screen and (min-width: 30em) {
 .f-6-ns, .f-headline-ns { font-size: 6rem; }
 .f-5-ns, .f-subheadline-ns { font-size: 5rem; }
 .f1-ns { font-size: 3rem; }
 .f2-ns { font-size: 2.25rem; }
 .f3-ns { font-size: 1.5rem; }
 .f4-ns { font-size: 1.25rem; }
 .f5-ns { font-size: 1rem; }
 .f6-ns { font-size: .875rem; }
}
@media screen and (min-width: 30em) and (max-width: 60em) {
 .f-6-m, .f-headline-m { font-size: 6rem; }
 .f-5-m, .f-subheadline-m { font-size: 5rem; }
 .f1-m { font-size: 3rem; }
 .f2-m { font-size: 2.25rem; }
 .f3-m { font-size: 1.5rem; }
 .f4-m { font-size: 1.25rem; }
 .f5-m { font-size: 1rem; }
 .f6-m { font-size: .875rem; }
}
@media screen and (min-width: 60em) {
 .f-6-l, .f-headline-l { font-size: 6rem; }
 .f-5-l, .f-subheadline-l { font-size: 5rem; }
 .f1-l { font-size: 3rem; }
 .f2-l { font-size: 2.25rem; }
 .f3-l { font-size: 1.5rem; }
 .f4-l { font-size: 1.25rem; }
 .f5-l { font-size: 1rem; }
 .f6-l { font-size: .875rem; }
}
```
- https://github.com/tachyons-css/tachyons-type-scale

## Flexbox
```css
/*

  FLEXBOX

  Media Query Extensions:
   -ns = not-small
   -m  = medium
   -l  = large

*/
.flex { display: flex; }
.inline-flex { display: inline-flex; }
/* 1. Fix for Chrome 44 bug.
 * https://code.google.com/p/chromium/issues/detail?id=506893 */
.flex-auto { flex: 1 1 auto; min-width: 0; /* 1 */ min-height: 0; /* 1 */ }
.flex-none { flex: none; }
.flex-column { flex-direction: column; }
.flex-row { flex-direction: row; }
.flex-wrap { flex-wrap: wrap; }
.items-start { align-items: flex-start; }
.items-end { align-items: flex-end; }
.items-center { align-items: center; }
.items-baseline { align-items: baseline; }
.items-stretch { align-items: stretch; }
.self-start { align-self: flex-start; }
.self-end { align-self: flex-end; }
.self-center { align-self: center; }
.self-baseline { align-self: baseline; }
.self-stretch { align-self: stretch; }
.justify-start { justify-content: flex-start; }
.justify-end { justify-content: flex-end; }
.justify-center { justify-content: center; }
.justify-between { justify-content: space-between; }
.justify-around { justify-content: space-around; }
.content-start { align-content: flex-start; }
.content-end { align-content: flex-end; }
.content-center { align-content: center; }
.content-between { align-content: space-between; }
.content-around { align-content: space-around; }
.content-stretch { align-content: stretch; }
.order-0 { order: 0; }
.order-1 { order: 1; }
.order-2 { order: 2; }
.order-3 { order: 3; }
.order-4 { order: 4; }
.order-5 { order: 5; }
.order-6 { order: 6; }
.order-7 { order: 7; }
.order-8 { order: 8; }
.order-last { order: 99999; }
@media screen and (min-width: 30em) {
 .flex-ns { display: flex; }
 .inline-flex-ns { display: inline-flex; }
 .flex-auto-ns { flex: 1 1 auto; min-width: 0; /* 1 */ min-height: 0; /* 1 */ }
 .flex-none-ns { flex: none; }
 .flex-column-ns { flex-direction: column; }
 .flex-row-ns { flex-direction: row; }
 .flex-wrap-ns { flex-wrap: wrap; }
 .items-start-ns { align-items: flex-start; }
 .items-end-ns { align-items: flex-end; }
 .items-center-ns { align-items: center; }
 .items-baseline-ns { align-items: baseline; }
 .items-stretch-ns { align-items: stretch; }
 .self-start-ns { align-self: flex-start; }
 .self-end-ns { align-self: flex-end; }
 .self-center-ns { align-self: center; }
 .self-baseline-ns { align-self: baseline; }
 .self-stretch-ns { align-self: stretch; }
 .justify-start-ns { justify-content: flex-start; }
 .justify-end-ns { justify-content: flex-end; }
 .justify-center-ns { justify-content: center; }
 .justify-between-ns { justify-content: space-between; }
 .justify-around-ns { justify-content: space-around; }
 .content-start-ns { align-content: flex-start; }
 .content-end-ns { align-content: flex-end; }
 .content-center-ns { align-content: center; }
 .content-between-ns { align-content: space-between; }
 .content-around-ns { align-content: space-around; }
 .content-stretch-ns { align-content: stretch; }
 .order-0-ns { order: 0; }
 .order-1-ns { order: 1; }
 .order-2-ns { order: 2; }
 .order-3-ns { order: 3; }
 .order-4-ns { order: 4; }
 .order-5-ns { order: 5; }
 .order-6-ns { order: 6; }
 .order-7-ns { order: 7; }
 .order-8-ns { order: 8; }
 .order-last-ns { order: 99999; }
}
@media screen and (min-width: 30em) and (max-width: 60em) {
 .flex-m { display: flex; }
 .inline-flex-m { display: inline-flex; }
 .flex-auto-m { flex: 1 1 auto; min-width: 0; /* 1 */ min-height: 0; /* 1 */ }
 .flex-none-m { flex: none; }
 .flex-column-m { flex-direction: column; }
 .flex-row-m { flex-direction: row; }
 .flex-wrap-m { flex-wrap: wrap; }
 .items-start-m { align-items: flex-start; }
 .items-end-m { align-items: flex-end; }
 .items-center-m { align-items: center; }
 .items-baseline-m { align-items: baseline; }
 .items-stretch-m { align-items: stretch; }
 .self-start-m { align-self: flex-start; }
 .self-end-m { align-self: flex-end; }
 .self-center-m { align-self: center; }
 .self-baseline-m { align-self: baseline; }
 .self-stretch-m { align-self: stretch; }
 .justify-start-m { justify-content: flex-start; }
 .justify-end-m { justify-content: flex-end; }
 .justify-center-m { justify-content: center; }
 .justify-between-m { justify-content: space-between; }
 .justify-around-m { justify-content: space-around; }
 .content-start-m { align-content: flex-start; }
 .content-end-m { align-content: flex-end; }
 .content-center-m { align-content: center; }
 .content-between-m { align-content: space-between; }
 .content-around-m { align-content: space-around; }
 .content-stretch-m { align-content: stretch; }
 .order-0-m { order: 0; }
 .order-1-m { order: 1; }
 .order-2-m { order: 2; }
 .order-3-m { order: 3; }
 .order-4-m { order: 4; }
 .order-5-m { order: 5; }
 .order-6-m { order: 6; }
 .order-7-m { order: 7; }
 .order-8-m { order: 8; }
 .order-last-m { order: 99999; }
}
@media screen and (min-width: 60em) {
 .flex-l { display: flex; }
 .inline-flex-l { display: inline-flex; }
 .flex-auto-l { flex: 1 1 auto; min-width: 0; /* 1 */ min-height: 0; /* 1 */ }
 .flex-none-l { flex: none; }
 .flex-column-l { flex-direction: column; }
 .flex-row-l { flex-direction: row; }
 .flex-wrap-l { flex-wrap: wrap; }
 .items-start-l { align-items: flex-start; }
 .items-end-l { align-items: flex-end; }
 .items-center-l { align-items: center; }
 .items-baseline-l { align-items: baseline; }
 .items-stretch-l { align-items: stretch; }
 .self-start-l { align-self: flex-start; }
 .self-end-l { align-self: flex-end; }
 .self-center-l { align-self: center; }
 .self-baseline-l { align-self: baseline; }
 .self-stretch-l { align-self: stretch; }
 .justify-start-l { justify-content: flex-start; }
 .justify-end-l { justify-content: flex-end; }
 .justify-center-l { justify-content: center; }
 .justify-between-l { justify-content: space-between; }
 .justify-around-l { justify-content: space-around; }
 .content-start-l { align-content: flex-start; }
 .content-end-l { align-content: flex-end; }
 .content-center-l { align-content: center; }
 .content-between-l { align-content: space-between; }
 .content-around-l { align-content: space-around; }
 .content-stretch-l { align-content: stretch; }
 .order-0-l { order: 0; }
 .order-1-l { order: 1; }
 .order-2-l { order: 2; }
 .order-3-l { order: 3; }
 .order-4-l { order: 4; }
 .order-5-l { order: 5; }
 .order-6-l { order: 6; }
 .order-7-l { order: 7; }
 .order-8-l { order: 8; }
 .order-last-l { order: 99999; }
}
```
- https://github.com/tachyons-css/tachyons-flexbox

## See Also
- [tachyons.io](http://tachyons.io/)
- [docs](http://tachyons.io/docs/)
- [components](http://tachyons.io/components/)
