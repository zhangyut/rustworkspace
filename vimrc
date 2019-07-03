"git clone https://github.com/gmarik/vundle.git ~/.vim/bundle/vundle
"git clone https://github.com/Valloric/YouCompleteMe.git ~/.vim/bundle/"
"cd .vim/bundle/YouCompleteMe; ./install.py --all"
"https://github.com/rust-lang/rust.vim"
"rustup component add rustfmt"
"rustup component add clippy"
":BundleInstall
set paste
set tabstop=4
set nocompatible               " be iMproved
filetype off                   " required!

set rtp+=~/.vim/bundle/vundle/
call vundle#rc()

" let Vundle manage Vundle
" required!
Bundle 'gmarik/vundle'
Bundle 'cespare/vim-golang'
Bundle 'Blackrush/vim-gocode'
Bundle 'majutsushi/tagbar'
Bundle 'scrooloose/nerdtree'
Bundle 'rust-lang/rust.vim'
:map <F5> :Tagbar<CR>
:map <F6> :NERDTree<CR>
let g:tagbar_type_go = {
    \ 'ctagstype' : 'go',
    \ 'kinds'     : [
        \ 'p:package',
        \ 'i:imports:1',
        \ 'c:constants',
        \ 'v:variables',
        \ 't:types',
        \ 'n:interfaces',
        \ 'w:fields',
        \ 'e:embedded',
        \ 'm:methods',
        \ 'r:constructor',
        \ 'f:functions'
    \ ],
    \ 'sro' : '.',
    \ 'kind2scope' : {
        \ 't' : 'ctype',
        \ 'n' : 'ntype'
    \ },
    \ 'scope2kind' : {
        \ 'ctype' : 't',
        \ 'ntype' : 'n'
    \ },
    \ 'ctagsbin'  : 'gotags',
    \ 'ctagsargs' : '-sort -silent'
        \ }
let g:rustfmt_autosave = 1
let g:rust_clip_command = 'pbcopy'
filetype plugin indent on
autocmd BufWritePre *.go :Fmt
syntax on
set fileencodings=utf-8,ucs-bom,gb18030,gbk,gb2312,cp936
set termencoding=utf-8
set encoding=utf-8
set paste
