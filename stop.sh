[[ $EUID != 0 ]] && echo "\\033[1;31mplease run this as root\\033[0m" && exit 1

if [ "$1" ]
then
    echo "\\033[1;31mstopping...\\033[0m"
    target_pid=$(pidof jayagra)
    if [ "$target_pid" ]
        then
            sudo kill $target_pid
            echo "\\033[1;32mkilled jayagra.com\\033[0m \\033[32m(jayagra.com $target_pid)\\033[0m"
        else
            echo "\\033[31mjayagra.com process not found\\033[0m"
    fi
else
    echo "please provide a parameter, \\033[1;32mstart\\033[0m or \\033[1;31mstop\\033[21m"
fi