%%
[0-9]+ printf("NUMBER: %s\n", yytext);
"+" |
"-" |
"*" |
"/" printf("OPERATOR: %s\n", yytext);
"(" printf("OPEN PARENTHESIS\n");
")" printf("CLOSED PARENTHESIS\n");
oui printf("ok merci\n");
"\n" printf("NEWLINE\n");
[[:blank:]] // ignore whitespaces
. printf("Invalid character: %c\n", *yytext);
