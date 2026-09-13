# Persona Exporter Types
[![Build and test](https://github.com/0DoubleDare/persona-exporter-types/actions/workflows/main.yml/badge.svg)](https://github.com/0DoubleDare/persona-exporter-types/actions/workflows/main.yml)
![GitHub repo size](https://img.shields.io/github/repo-size/0DoubleDare/persona-exporter-types)

`persona-exporter-types` это крейт который предоставляет типы данных для `persona-exporter`.

Crate используется как единый источник структур, перечислений и реализаций, которые применяются при обмене данными между
компонентами проекта. Это снижает дублирование моделей и упрощает сопровождение совместимости.

Другие разработчики могут использовать эти же структуры в собственных Rust-проектах, включая веб-сайты, сервисы или даже собственные
экспортеры метрик если моя структура вам покашется цельной и правильной.


### Установка

Добавить зависимость в `Cargo.toml`:

```toml
[dependencies]
persona-exporter-types = { version = "2.1.0", features = "full" }
```
#### Доступные features:
- `serde`: Включает поддержку сериализации и десирилизации
- `line-protocol`: Включает поддержку трейта From для крейта `influxdb-line-protocol`
- `from-trait-sysinfo`: Преобразование структур из Sysinfo в локальные
- `convert-data-unit`: Преобразование единиц измерения. К примеру из килобайт в мегабайты, из гигабайт в мегабайты.
  Конвертация во все стороны работает для следующих единиц измерения: Байты, Килобайты, Мегабайты, Гигабайты, Терабайты и Петабайты.
- `default`: Фичи по умолчанию, содержат `serde` и `convert-data-unit`
- `full`: Включает всё

### Использование

Подключить crate в коде:

```rust
use persona_exporter_types::*;

// Структуры метрик
use persona_exporter_types::metrics::*;

// Типы и трейты: ConvertTo / DataUnit - конвертация единицы измерения памяти
use persona_exporter_types::types::*;
```

### Версионирование

Используется схема `MAJOR.MINOR.PATCH` (пример: `1.0.0`).

- `MAJOR` — главное обновление; может содержать несовместимые изменения и требовать адаптации кода.
- `MINOR` — небольшое функциональное обновление; обычно обратно совместимо, но может нести интеграционные риски.
- `PATCH` — безопасный патч; исправления и мелкие улучшения без изменения публичного контракта.

Пример `1.3.2`:

- `1` — major-уровень,
- `3` — minor-уровень,
- `2` — patch-уровень.
