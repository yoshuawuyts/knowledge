# LaTeX
Word processor

## Tools
- latexmk
- latexrun
- pulp

## Example Makefile
```make
TEX=pdflatex -shell-escape -interaction=nonstopmode -file-line-error
PRE= $(TEX) -ini -job-name="preamble" "&pdflatex preamble.tex\dump"
BIB=bibtex
NAME=template

ifeq ($(OS),Windows_NT)
	VIEWER=start
else
    UNAME_S := $(shell uname -s)
    ifeq ($(UNAME_S),Linux)
        VIEWER=xdg-open
    endif
    ifeq ($(UNAME_S),Darwin)
        VIEWER=open
    endif
endif

# Test if latexmk is available and not buggy (like on my Windows machine...)
HAS_LATEXMK=FALSE
ifneq (,$(shell which latexmk))
ifeq (0,$(shell latexmk --help > /dev/null 2>&1; echo $$?))
	HAS_LATEXMK=TRUE
endif
endif

.PHONY: all view clean watch verbose rebuild mrproper run pulp
.SILENT:

view : $(NAME).pdf
	@$(VIEWER) $(NAME).pdf

pdf : $(NAME).pdf

ifeq ($(HAS_LATEXMK),TRUE)

$(NAME).pdf: $(NAME).tex $(NAME).bib
	@latexmk -pdf -quiet $(NAME).tex
	@echo "/!\ In case of error, run 'make verbose'"

verbose:
	@latexmk -pdf -gg $(NAME).tex

rebuild:
	@latexmk -pdf -quiet -gg $(NAME).tex

clean:
	@latexmk -c $(NAME).tex
	@rm -f $(NAME).vtc $(NAME).synctex.gz comment.cut
	@rm -rf auto latex.out

watch:
	@latexmk -pdf -pvc $(NAME).tex

else # latexmk unavailable

# adapted from: http://tex.stackexchange.com/questions/40738/

$(NAME).pdf : $(NAME).tex $(NAME).bbl $(NAME).blg
	echo yay
	$(TEX) $(NAME).tex

$(NAME).bbl $(NAME).blg : $(NAME).bib $(NAME).aux
	$(BIB) $(NAME)

$(NAME).aux: $(NAME).tex
	$(TEX) $(NAME).tex

clean:
	@rm -f $(NAME).aux $(NAME).bbl $(NAME).blg $(NAME).log $(NAME).out \\
		$(NAME).vtc $(NAME).fdb_latexmk $(NAME).fls $(NAME).synctex.gz latex.out comment.cut
	@rm -rf auto latex.out

endif

run:
	./latexrun $(NAME).tex

pulp:
	pulp $(NAME).log

mrproper: clean
	@rm -f $(NAME).pdf
```

## See Also
- http://norswap.com/latex-tooling/
