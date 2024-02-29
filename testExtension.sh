#!/bin/bash

rm -rf latex
mkdir latex latex/grammars latex/languages latex/languages/latex

cp extension.json latex
cp grammars/latex.wasm latex/grammars
cp -r languages/latex/* latex/languages/latex
