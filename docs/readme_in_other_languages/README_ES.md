<p align="center">
  <img src="https://github.com/user-attachments/assets/99a8117c-bd7e-4633-b6bb-3f6ce2c29bcb" alt="Renux Logo">
</p>

#

Renux OS es un sistema operativo completo similar a Unix escrito en Rust. Este proyecto tiene como objetivo proporcionar un kernel híbrido que se pueda ampliar con características adicionales según sea necesario. El enfoque principal es aprender y explorar el desarrollo de un sistema operativo en Rust.

## Características [En planificación]

- [X] **Escrito en Rust**: Aprovechando las características de seguridad y concurrencia de Rust.
- [ ] **Kernel Híbrido**: Combina elementos de diseños monolíticos y de microkernel.
- [X] **Script en Python**: Facilita la construcción de Renux.
- [ ] **Lenguajes C y C++**: Agregar herramientas y controladores en C y C++ para comunicarse directamente con el hardware.
- [X] **Imagen de Arranque**: Generar una imagen de arranque utilizando `cargo bootimage`.
- [X] **Compatibilidad con QEMU**: Permite probar el sistema operativo en un entorno virtual utilizando QEMU.

## Estado de Desarrollo
> [!WARNING]
> Renux OS está actualmente en fase de desarrollo. Muchas características todavía están siendo implementadas y probadas. Se aceptan contribuciones y comentarios para mejorar y expandir el proyecto.

## Primeros Pasos

### Prerrequisitos

Para compilar y ejecutar Renux OS, necesitas tener las siguientes herramientas instaladas:

- [Rust](https://www.rust-lang.org/): Instala Rust usando `rustup`.
- `cargo bootimage`: Instálalo con `cargo install bootimage --version "^0.10.0"`.
- [QEMU](https://www.qemu.org/): Opcional, para emular el sistema operativo.

### Compilar el Sistema Operativo

1. **Clonar el Repositorio**:

    ```sh
    git clone https://github.com/Renan2010/renuxos.git
    cd renuxos
    ```

2. **Instalar Rust Nightly y Componentes**:

    ```sh
    rustup install nightly
    rustup component add rust-src --toolchain nightly
    ```

3. **Instalar `bootimage`**:

    ```sh
    cargo install bootimage --version "^0.10.0"
    ```

4. **Compilar la Imagen de Arranque**

    Modo fácil:
    ```sh
    python build.py
    ```
    Modo avanzado:
    ```sh
    cargo bootimage --target x86_64-unknown-none -j núcleos # número de núcleos de CPU
    ```

6. **Ejecutar con QEMU** (opcional):

    ```sh
    qemu-system-x86_64 -drive format=raw,file=target/x86_64-unknown-none/debug/bootimage-renuxos.bin
    ```
