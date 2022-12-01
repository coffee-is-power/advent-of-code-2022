pacman -Syyuu \
  curl \
  git \
  gnupg \
  jq \
  sudo \
  zsh \
  vim \
  base-devel \
  openssl
curl https://sh.rustup.rs -sSf | sh -s -- -y 
source "$HOME/.cargo/env"
rustup install nightly
rustup component add rustfmt
rustup component add rustfmt --toolchain nightly
rustup component add clippy 
rustup component add clippy --toolchain nightly

cargo install cargo-expand
cargo install cargo-edit

## setup and install oh-my-zsh
USERNAME=$(whoami)
USER_UID=$(id -u)
USER_GID=$(id -g)
sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
cp -R /root/.oh-my-zsh /home/$USERNAME
cp /root/.zshrc /home/$USERNAME
sed -i -e "s/\/root\/.oh-my-zsh/\/home\/$USERNAME\/.oh-my-zsh/g" /home/$USERNAME/.zshrc
chown -R $USER_UID:$USER_GID /home/$USERNAME/.oh-my-zsh /home/$USERNAME/.zshrc