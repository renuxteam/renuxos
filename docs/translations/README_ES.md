<p align="center">
  <img src="https://github.com/user-attachments/assets/99a8117c-bd7e-4633-b6bb-3f6ce2c29bcb" alt="Renux Logo">
</p>
<p align="center">
 <img src="https://github.com/user-attachments/assets/9e11d6c0-ac41-42bf-ae24-73bd1c37ab6d" alt="Renux OS">
</p>

# **Renux OS**

Renux OS es un **sistema operativo tipo Unix** completo, escrito en **Rust**. Este proyecto tiene como objetivo proporcionar un **núcleo híbrido** que puede ser ampliado con características adicionales según sea necesario. El enfoque principal es aprender y explorar el desarrollo de un sistema operativo en **Rust**.

## 🚀 **Planes Futuros**

Para ver nuestros planes y el futuro del sistema Renux, haz clic [aquí](docs/plans/future_plans.md).

## 🛠️ **Estado del Desarrollo**
> [!WARNING]
> Renux OS está actualmente en la fase de **desarrollo**. Muchas características aún están siendo implementadas y probadas. Se aceptan contribuciones y comentarios para ayudar a mejorar y expandir el proyecto.

## 📝 **Construir este SO y Documentación**
Para obtener instrucciones detalladas sobre cómo construir el sistema operativo, incluida la documentación, visita el directorio [docs](./docs/docs.md).

## 🌍 **Leer el Readme en Otros Idiomas**
- 🇧🇷 **Português** | [README_BR.md](./README_BR.md)
- 🇺🇸 **English** | [README.md](../../README.md)

---

## 🚀 **Versiones Futuras**
- **Versión 1.0 ("Aurora")**: Lanzamiento inicial para pruebas internas y comentarios de la comunidad.
- **Versión 2.0 ("Jaguar")**: Lanzamiento público con más características y mayor estabilidad.
- **Versiones subsecuentes**: Alternando entre **pruebas internas** y **lanzamientos públicos** para asegurar la evolución continua del sistema.

## 🔐 **Seguridad**
- **Mejorar el núcleo en Rust**: Implementar capas de seguridad adicionales y control de memoria con Rust.
- **Añadir drivers seguros**: Enfocarse en la seguridad de los drivers, haciendo la transición de C a Zig para aumentar la seguridad y eficiencia.
- **Monitoreo inteligente de fallos**: Desarrollar un sistema automatizado de detección y recuperación de fallos.

## ⚡ **Desempeño**
- **Optimización de código y memoria**: Continuar trabajando en asignadores personalizados para reducir la fragmentación de memoria.
- **Driver moderno en Zig**: Usar Zig para crear drivers que optimicen el rendimiento manteniendo la seguridad de la memoria.
- **Compilación más rápida y eficiente**: Mejorar los procesos de compilación usando Cargo y Zig para reducir el tiempo de compilación y mejorar la modularidad.

## 🛠️ **Infraestructura y Herramientas**
- **Menuconfig**: Desarrollar un sistema de configuración separado para Renux OS, similar al `menuconfig` de Linux, pero independiente del sistema operativo.
- **Sistema de actualización inteligente**: Desarrollar un sistema de actualización automatizado para drivers y el núcleo, facilitando actualizaciones continuas sin necesidad de reconstruir todo el sistema.

## 🔄 **Integración de Lenguajes**
- **Zig, Rust, C y C++ en el sistema**: Continuar integrando estos lenguajes de manera efectiva, aprovechando los puntos fuertes de cada uno:
  - **Rust** para el núcleo y gestión segura de memoria.
  - **Zig** para drivers modernos, cross-compilation y optimizaciones de bajo nivel.
  - **C/C++** para drivers de hardware legados y compatibilidad.

## 🌍 **Expansión y Contribuciones**
- **Ecosistema open-source**: Continuar la evolución de Renux OS como un proyecto open-source, fomentando contribuciones de desarrolladores y construyendo una comunidad fuerte.
- **Alianzas y colaboraciones**: Explorar oportunidades para colaborar con otras comunidades de sistemas operativos y proyectos open-source para compartir ideas y mejorar Renux OS.

## 🗂️ **Base de Código Descentralizada con Submódulos**
La base de código de Renux OS está diseñada para ser **descentralizada** mediante el uso de **submódulos Git**. Esto permite separar diferentes componentes del sistema operativo en **repositorios independientes**, mientras se mantiene un proceso de compilación unificado. Al usar submódulos, la arquitectura del sistema puede evolucionar de manera más flexible, con cada módulo siendo desarrollado y mantenido independientemente, pero fácilmente integrado al código principal.

## 💡 **Objetivos a Largo Plazo**
- **Desarrollar una arquitectura modular** que permita actualizaciones rápidas y seguras.
- **Expandir la compatibilidad** con diversas arquitecturas y dispositivos, incluyendo sistemas embebidos y dispositivos de bajo consumo.
- **Enfocarse en la innovación continua**, superando los sistemas operativos tradicionales con **seguridad mejorada**, **rendimiento excepcional** y **experiencia de usuario optimizada**.

## 🚀 **Conclusión**
Renux OS está diseñado para ser un sistema operativo **moderno, eficiente y seguro**. La integración de lenguajes como Rust, Zig, C y C++ ofrece una flexibilidad increíble, mientras que el enfoque en **seguridad** y **desempeño** garantiza que el sistema se convertirá en una **referencia para el futuro de los sistemas operativos**.
