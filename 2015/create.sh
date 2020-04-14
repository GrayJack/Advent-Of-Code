#!/bin/env sh

mkdir $1
mkdir $1/src
echo '(declare-project' >> $1/project.janet
echo '    :name "'$1'"' >> $1/project.janet
echo '    :description "'$1' of Avent of code 2015"' >> $1/project.janet
echo '    :author "Eric Shimizu Karbstein"' >> $1/project.janet
echo '    :license "Unlicense")' >> $1/project.janet
