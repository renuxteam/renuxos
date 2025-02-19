<p align="center">
  <img src="https://github.com/user-attachments/assets/99a8117c-bd7e-4633-b6bb-3f6ce2c29bcb" alt="Renux Logo">
</p>

#

Renux OS é um sistema operacional completo do tipo Unix escrito em Rust. Este projeto visa fornecer um kernel híbrido que pode ser estendido com recursos adicionais conforme necessário. O principal foco é aprender e explorar o desenvolvimento de um sistema operacional em Rust.

## Funcionalidades [Em planejamento]

- [X] **Escrito em Rust**: Aproveitando os recursos de segurança e concorrência do Rust.
- [ ] **Kernel Híbrido**: Combina elementos de designs monolíticos e de microkernel.
- [X] **Script Python**: Construa o Renux facilmente
- [ ] **Linguagens C e C++**: Adicione ferramentas e drivers em C e C++ para se comunicar diretamente com o hardware
- [X] **Imagem Bootável**: Crie uma imagem bootável usando `cargo bootimage`.
- [X] **Suporte QEMU**: Teste seu sistema operacional em um ambiente virtual usando QEMU.

## Status de Desenvolvimento
> [!WARNING]
> Renux OS está atualmente em fase de desenvolvimento. Muitos recursos ainda estão sendo implementados e testados. Contribuições e feedback são bem-vindos para ajudar a melhorar e expandir o projeto.

## Primeiros Passos

### Pré-requisitos

Para construir e executar o Renux OS, você precisa ter as seguintes ferramentas instaladas:

- [Rust](https://www.rust-lang.org/): Instale Rust usando `rustup`.
- `cargo bootimage`: Instale usando `cargo install bootimage --version "^0.10.0"`.
- [QEMU](https://www.qemu.org/): Opcional, para emular o sistema operacional.

### Construa este SO

1. **Clone o Repositório**:

    ```sh
    git https://github.com/Renan2010/renuxos.git
    cd renuxos
    ```

2. **Instale Rust Nightly e Componentes**:

    ```sh
    rustup install nightly
    rustup component add rust-src --toolchain nightly
    ```

3. **Instale `bootimage`**:

    ```sh
    cargo install bootimage --version "^0.10.0"
    ```

4. **Construa a Imagem Bootável**

    Modo fácil
    ```sh
    python build.py
    ```
    Modo difícil
    ```sh
    cargo bootimage --target x86_64-unknown-none -j cores # número de núcleos da CPU
    ```

6. **Execute com QEMU** (opcional):

    ```sh
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-renuxos.bin
    ```
