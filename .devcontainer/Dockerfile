FROM ubuntu:jammy as generator
    # 

FROM ubuntu:jammy
RUN apt update && \
    apt install -y curl build-essential llvm sudo pkg-config libssl-dev openssh-client zsh git
RUN sh -c "$(curl -fsSL https://raw.githubusercontent.com/ohmyzsh/ohmyzsh/master/tools/install.sh)" "" --unattended && \
sed -i 's/ZSH_THEME="robbyrussell"/ZSH_THEME="agnoster"/g' ~/.zshrc
RUN bash -c "$(curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs)" -q -y --default-toolchain stable --default-host x86_64-unknown-linux-gnu --profile default
ENTRYPOINT [ "zsh" ]
