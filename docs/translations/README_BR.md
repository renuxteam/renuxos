<p align="center">
  <img src="https://github.com/user-attachments/assets/99a8117c-bd7e-4633-b6bb-3f6ce2c29bcb" alt="Renux Logo">
</p>
<p align="center">
 <img src="https://github.com/user-attachments/assets/00c385c8-7796-4a60-80b9-b40b496358fc" alt="Renux OS">
</p>

# **Renux OS**

O Renux OS é um **sistema operacional tipo Unix** completo, escrito em **Rust**. Este projeto tem como objetivo fornecer um **kernel híbrido** que pode ser expandido com recursos adicionais conforme necessário. O foco principal é aprender e explorar o desenvolvimento de um sistema operacional em **Rust**.

## 🚀 **Planos Futuros**

Para ver nossos planos e o futuro do sistema Renux, clique [aqui](docs/plans/future_plans.md).

## 🛠️ **Status do Desenvolvimento**
> [!WARNING]
> O Renux OS está atualmente em fase de **desenvolvimento**. Muitos recursos ainda estão sendo implementados e testados. Contribuições e feedback são bem-vindos para ajudar a melhorar e expandir o projeto.

## 📝 **Construir este SO e Documentação**
Para instruções detalhadas sobre como construir o sistema operacional, incluindo a documentação, visite o diretório [docs](./docs/docs.md).

## 🌍 **Leia o Readme em Outras Línguas**
- 🇪🇸 **Español** | [README_ES.md](./docs/translations/README_ES.md)
- 🇺🇸 **English** | [README.md](./README.md)

---

## 🚀 **Versões Futuras**
- **Versão 1.0 ("Aurora")**: Lançamento inicial para testes internos e feedback da comunidade.
- **Versão 2.0 ("Jaguar")**: Lançamento público com mais recursos e maior estabilidade.
- **Versões subsequentes**: Alternando entre **testes internos** e **lançamentos públicos** para garantir a evolução contínua do sistema.

## 🔐 **Segurança**
- **Aprimorar o Kernel em Rust**: Implementar camadas de segurança adicionais e controle de memória com Rust.
- **Adicionar drivers seguros**: Focar na segurança dos drivers, fazendo a transição de C para Zig para aumentar a segurança e eficiência.
- **Monitoramento inteligente de falhas**: Desenvolver um sistema automatizado de detecção e recuperação de falhas.

## ⚡ **Desempenho**
- **Otimização de código e memória**: Continuar trabalhando em alocadores personalizados para reduzir a fragmentação de memória.
- **Driver moderno em Zig**: Usar Zig para criar drivers que otimizem o desempenho mantendo a segurança de memória.
- **Compilação mais rápida e eficiente**: Melhorar os processos de build usando Cargo e Zig para reduzir o tempo de compilação e melhorar a modularidade.

## 🛠️ **Infraestrutura e Ferramentas**
- **Menuconfig**: Desenvolver um sistema de configuração separado para o Renux OS, similar ao `menuconfig` do Linux, mas independente do sistema operacional.
- **Bootloader em Zig**: Criar um bootloader mais eficiente e personalizado usando Zig para melhorar o tempo de inicialização e a detecção de hardware.
- **Sistema de atualização inteligente**: Desenvolver um sistema de atualização automatizado para drivers e o kernel, facilitando atualizações contínuas sem a necessidade de reconstruir o sistema inteiro.

## 🔄 **Integração de Linguagens**
- **Zig, Rust, C e C++ no sistema**: Continuar integrando essas linguagens de maneira eficaz, aproveitando os pontos fortes de cada uma:
  - **Rust** para o Kernel e gerenciamento seguro de memória.
  - **Zig** para drivers modernos, cross-compilation e otimizações de baixo nível.
  - **C/C++** para drivers de hardware legados e compatibilidade.

## 🌍 **Expansão e Contribuições**
- **Ecossistema open-source**: Continuar a evolução do Renux OS como um projeto open-source, incentivando contribuições de desenvolvedores e construindo uma comunidade forte.
- **Parcerias e colaborações**: Explorar oportunidades de colaboração com outras comunidades de sistemas operacionais e projetos open-source para compartilhar ideias e melhorar o Renux OS.

## 🗂️ **Base de Código Descentralizada com Submódulos**
A base de código do Renux OS foi projetada para ser **descentralizada** por meio do uso de **submódulos Git**. Isso permite a separação de diferentes componentes do sistema operacional em **repositórios independentes**, mantendo um processo de build unificado. Ao usar submódulos, a arquitetura do sistema pode evoluir de forma mais flexível, com cada módulo sendo desenvolvido e mantido independentemente, mas facilmente integrado ao código principal.

## 💡 **Objetivos de Longo Prazo**
- **Desenvolver uma arquitetura modular** que permita atualizações rápidas e seguras.
- **Expandir a compatibilidade** com várias arquiteturas e dispositivos, incluindo sistemas embarcados e dispositivos de baixo consumo.
- **Focar na inovação contínua**, superando os sistemas operacionais tradicionais com **segurança aprimorada**, **desempenho excepcional** e **experiência otimizada para o usuário**.

## 🚀 **Conclusão**
O Renux OS foi projetado para ser um sistema operacional **moderno, eficiente e seguro**. A integração de linguagens como Rust, Zig, C e C++ oferece flexibilidade incrível, enquanto o foco em **segurança** e **desempenho** garante que o sistema se tornará uma **referência para o futuro dos sistemas operacionais**.
