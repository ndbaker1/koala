// // FSM based lexer

export interface Token {
  name: string
  value: any
}

export type Lexer = {
  lex: (code: string) => Token[]
}

export function createLexer(tokenPatterns: Record<string, RegExp>): Lexer {
  const lex = (code: string) => { return [] }
  return { lex }
}

// class Lexer {
//   constructor(private tokenPatterns: Record<string | "EMPTY", RegExp>) { }

//   lex(programCode: string) {
//     // What we'll produce
//     const tokens = []
//     let iterator = 0

//     while (iterator != programCode.length) {
//       // Skip over whitespaces
//       while (iterator != programCode.length && this.tokenPatterns.EMPTY.test(programCode[iterator]))
//         iterator++

//       // Caching the current char because we will use it A LOT
//       const currentChar = programCode[iterator]

//       switch (currentChar) {
//         case this.tokenPatterns:
//           ++programIt;
//           tokens.push(Token:: COMMA);
//           break;

//         // Lots of similar cases for simple one character tokens

//         default:
//           // We have a token coming from a more complex pattern,
//           // let's find out what
//           std::string currentStr;

//           // If this starts with a letter,
//           // it is either a keyword or an identifier
//           if (isalpha(currentChar, loc)) {
//             // Let's greedily build a string with the pattern
//             currentStr = currentChar;
//             // We accept alphanumeric characters because it is legal for a
//             // function or variable identifier to contains number
//             while (++programIt != programEnd && isalnum(* programIt, loc))
//               currentStr += * programIt;

//             if (currentStr == "fn")
//               toks.push_back(Token:::: FUNCTION);
//             // Other cases where the pattern directly match
//             // a keyword token...
//             else
//               // If this doesn't match a keyword,
//               // then it is a simple identifier
//               toks.push_back(Token::: IDENTIFIER);
//           }
//           // It starts with a digit, then it can only be a number
//           else if (isdigit(currentChar, loc)) {
//             // Eat up all the digits making up the number
//             while (isdigit(* (++programIt, loc)));

//             toks.push_back(Token:: NUMBER);
//           }
//           else {
//             // The hell is that?
//             toks.push_back(Token:: UNKNOWN);
//           }
//       }
//       iterator++
//     }
//   }
// }
