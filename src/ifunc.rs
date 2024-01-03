use crate::vm::ifunc;
use crate::vm::VM;
// 内蔵関数の定義

// atan2(y,x)
//     Return arctangent of y/x in radians in the range [-,].
// cos(x)
//     Return cosine of x, where x is in radians.
// sin(x)
//     Return sine of x, where x is in radians.
// exp(x)
//     Return the exponential function of x.
// log(x)
//     Return the natural logarithm of x.
// sqrt(x)
//     Return the square root of x.
// int(x)
//     Return the argument truncated to an integer. Truncation shall be toward 0 when x>0.
// rand()
//     Return a random number n, such that 0<=n<1.
// srand([expr])
//     Set the seed value for rand to expr or use the time of day if expr is omitted. The previous seed value shall be returned.
//
// String Functions
//
// The string functions in the following list shall be supported. Although the grammar (see Grammar ) permits built-in functions to appear with no arguments or parentheses, unless the argument or parentheses are indicated as optional in the following list (by displaying them within the "[]" brackets), such use is undefined.
//
// gsub(ere, repl[, in])
//     Behave like sub (see below), except that it shall replace all occurrences of the regular expression (like the ed utility global substitute) in $0 or in the in argument, when specified.
// index(s, t)
//     Return the position, in characters, numbering from 1, in string s where string t first occurs, or zero if it does not occur at all.
// length[([s])]
//     Return the length, in characters, of its argument taken as a string, or of the whole record, $0, if there is no argument.
// match(s, ere)
//     Return the position, in characters, numbering from 1, in string s where the extended regular expression ere occurs, or zero if it does not occur at all. RSTART shall be set to the starting position (which is the same as the returned value), zero if no match is found; RLENGTH shall be set to the length of the matched string, -1 if no match is found.
// split(s, a[, fs  ])
//     Split the string s into array elements a[1], a[2], ..., a[n], and return n. All elements of the array shall be deleted before the split is performed. The separation shall be done with the ERE fs or with the field separator FS if fs is not given. Each array element shall have a string value when created and, if appropriate, the array element shall be considered a numeric string (see Expressions in awk). The effect of a null string as the value of fs is unspecified.
// sprintf(fmt, expr, expr, ...)
//     Format the expressions according to the printf format given by fmt and return the resulting string.
// sub(ere, repl[, in  ])
//     Substitute the string repl in place of the first instance of the extended regular expression ERE in string in and return the number of substitutions. An <ampersand> ( '&' ) appearing in the string repl shall be replaced by the string from in that matches the ERE. An <ampersand> preceded with a <backslash> shall be interpreted as the literal <ampersand> character. An occurrence of two consecutive <backslash> characters shall be interpreted as just a single literal <backslash> character. Any other occurrence of a <backslash> (for example, preceding any other character) shall be treated as a literal <backslash> character. Note that if repl is a string literal (the lexical token STRING; see Grammar), the handling of the <ampersand> character occurs after any lexical processing, including any lexical <backslash>-escape sequence processing. If in is specified and it is not an lvalue (see Expressions in awk), the behavior is undefined. If in is omitted, awk shall use the current record ($0) in its place.
// substr(s, m[, n  ])
//     Return the at most n-character substring of s that begins at position m, numbering from 1. If n is omitted, or if n specifies more characters than are left in the string, the length of the substring shall be limited by the length of the string s.
// tolower(s)
//     Return a string based on the string s. Each character in s that is an uppercase letter specified to have a tolower mapping by the LC_CTYPE category of the current locale shall be replaced in the returned string by the lowercase letter specified by the mapping. Other characters in s shall be unchanged in the returned string.
// toupper(s)
//     Return a string based on the string s. Each character in s that is a lowercase letter specified to have a toupper mapping by the LC_CTYPE category of the current locale is replaced in the returned string by the uppercase letter specified by the mapping. Other characters in s are unchanged in the returned string.
//
// All of the preceding functions that take ERE as a parameter expect a pattern or a string valued expression that is a regular expression as defined in Regular Expressions.
// Input/Output and General Functions
//
// The input/output and general functions are:
//
// close(expression)
//     Close the file or pipe opened by a print or printf statement or a call to getline with the same string-valued expression. The limit on the number of open expression arguments is implementation-defined. If the close was successful, the function shall return zero; otherwise, it shall return non-zero.
// expression |  getline [var]
//     Read a record of input from a stream piped from the output of a command. The stream shall be created if no stream is currently open with the value of expression as its command name. The stream created shall be equivalent to one created by a call to the popen() function with the value of expression as the command argument and a value of r as the mode argument. As long as the stream remains open, subsequent calls in which expression evaluates to the same string value shall read subsequent records from the stream. The stream shall remain open until the close function is called with an expression that evaluates to the same string value. At that time, the stream shall be closed as if by a call to the pclose() function. If var is omitted, $0 and NF shall be set; otherwise, var shall be set and, if appropriate, it shall be considered a numeric string (see Expressions in awk).
//
//     The getline operator can form ambiguous constructs when there are unparenthesized operators (including concatenate) to the left of the '|' (to the beginning of the expression containing getline). In the context of the '$' operator, '|' shall behave as if it had a lower precedence than '$'. The result of evaluating other operators is unspecified, and conforming applications shall parenthesize properly all such usages.
// getline
//     Set $0 to the next input record from the current input file. This form of getline shall set the NF, NR, and FNR variables.
// getline  var
//     Set variable var to the next input record from the current input file and, if appropriate, var shall be considered a numeric string (see Expressions in awk). This form of getline shall set the FNR and NR variables.
// getline [var]  < expression
//     Read the next record of input from a named file. The expression shall be evaluated to produce a string that is used as a pathname. If the file of that name is not currently open, it shall be opened. As long as the stream remains open, subsequent calls in which expression evaluates to the same string value shall read subsequent records from the file. The file shall remain open until the close function is called with an expression that evaluates to the same string value. If var is omitted, $0 and NF shall be set; otherwise, var shall be set and, if appropriate, it shall be considered a numeric string (see Expressions in awk).
//
//     The getline operator can form ambiguous constructs when there are unparenthesized binary operators (including concatenate) to the right of the '<' (up to the end of the expression containing the getline). The result of evaluating such a construct is unspecified, and conforming applications shall parenthesize properly all such usages.
// system(expression)
//     Execute the command given by expression in a manner equivalent to the system() function defined in the System Interfaces volume of POSIX.1-2017 and return the exit status of the command.
//
// All forms of getline shall return 1 for successful input, zero for end-of-file, and -1 for an error.
//
// Where strings are used as the name of a file or pipeline, the application shall ensure that the strings are textually identical. The terminology "same string value" implies that "equivalent strings", even those that differ only by <space> characters, represent different files.

type Func = fn(vm: &mut VM);
struct IFunc {
    name: &'static str,
    func: Func,
    arglen: usize,
}

const INTERNAL_FUNC: &[IFunc] = &[
    IFunc {
        name: "sin",
        func: ifunc::ifunc_sin,
        arglen: 1,
    },
    IFunc {
        name: "cos",
        func: ifunc::ifunc_cos,
        arglen: 1,
    },
    IFunc {
        name: "exp",
        func: ifunc::ifunc_exp,
        arglen: 1,
    },
    IFunc {
        name: "tolower",
        func: ifunc::ifunc_tolower,
        arglen: 1,
    },
    IFunc {
        name: "toupper",
        func: ifunc::ifunc_toupper,
        arglen: 1,
    },
    IFunc {
        name: "rand",
        func: ifunc::ifunc_rand,
        arglen: 0,
    },
];

pub fn get_index_from_name(name: &str) -> Option<usize> {
    INTERNAL_FUNC.iter().position(|i| i.name == name)
}

pub fn call_internal_func_from_index(index: usize, vm: &mut VM) {
    (INTERNAL_FUNC[index].func)(vm);
}

pub fn get_len_of_args(index: usize) -> usize {
    INTERNAL_FUNC[index].arglen
}

#[test]
fn test_index_from_name() {
    assert_eq!(0, get_index_from_name("sin").unwrap());
}
