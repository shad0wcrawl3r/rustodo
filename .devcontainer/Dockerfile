FROM ubuntu:jammy as generator
RUN apt update && apt install sudo openssl curl git zsh -y && \
    useradd -m -N -s /usr/bin/zsh -u 1000 -p '$(openssl passwd -1 "password123")' vscode-dev && \
    usermod -aG sudo vscode-dev  # Grant sudo to the user
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended 
    # sed -i 's/ZSH_THEME="robbyrussell"/ZSH_THEME="agnoster"/g' ~/.zshrc

FROM ubuntu:jammy as product
COPY --from=generator /etc/passwd /etc/shadow /etc/group /etc/
COPY --from=generator --chown=1000:1000 /root/ /home/vscode-dev/
RUN apt update && apt install -y curl build-essential llvm sudo pkg-config libssl-dev zsh git && chown vscode-dev /home/vscode-dev
USER vscode-dev
RUN bash -c "$(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs)" -q -y --default-toolchain stable --default-host x86_64-unknown-linux-gnu --profile default
ENTRYPOINT [ "zsh" ]
