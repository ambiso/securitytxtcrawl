FROM archlinux
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs > rustup.sh
RUN sh rustup.sh -y
RUN pacman -Sy base-devel --noconfirm
ADD Cargo.toml .
ADD Cargo.lock .
ADD src src
ENV PATH="/root/.cargo/bin:${PATH}"
RUN cargo build --release
RUN rm -rf /var/cache
CMD cargo run --release
