# Linera Passport NFT - Deployment Guide

## Overview

Linera Passport NFT - это приложение для Linera Buildathon, которое позволяет пользователям:
- Минтить уникальный NFT паспорт (один на адрес)
- Зарабатывать скор за активность в сети Linera Conway
- Автоматически обновлять достижения через оракул

## Архитектура

```
┌─────────────────────────────────────────────────────────────────┐
│                     LINERA BLOCKCHAIN                            │
│  ┌──────────────────┐        ┌──────────────────┐              │
│  │ Passport NFT     │        │  Linera Service  │              │
│  │   Contract       │◄───────│   (Port 8080)    │              │
│  └──────────────────┘        └──────────────────┘              │
└─────────────────────────────────────────────────────────────────┘
           ▲                              │
           │                              ▼
    UpdateAchievements          ┌──────────────────┐
           │                     │ Linera Indexer   │
           │                     │   (Port 8000)    │
           │                     └──────────────────┘
           │                              │
           │                              │ Query activity
           │                              ▼
    ┌──────────────────┐         ┌──────────────────┐
    │  Passport Agent  │◄────────│  GraphQL API     │
    │    (Oracle)      │         └──────────────────┘
    └──────────────────┘
           ▲
           │
    User triggers update
           │
    ┌──────────────────┐
    │   Web Frontend   │
    │  (Next.js app)   │
    └──────────────────┘
```

## Система Скоринга

### Базовый скор
- **10 транзакций = 1 балл**
  - Считаются все операции: transfers + user operations

### Достижения (Achievements)

| Код | Описание | Баллы | Условие |
|-----|----------|-------|---------|
| `CONWAY_PARTICIPANT` | Участие в тестовой сети Conway | 100 | Минимум 1 user_operation |
| `APP_CREATOR` | Создание приложения в сети | 100 | Обнаружена операция создания app |
| `TRANSACTION_MILESTONE_10` | 10+ транзакций | 0 | 10+ total transactions |
| `TRANSACTION_MILESTONE_50` | 50+ транзакций | 0 | 50+ total transactions |
| `TRANSACTION_MILESTONE_100` | 100+ транзакций | 0 | 100+ total transactions |

### Пример расчета
```
Пользователь:
- 45 transactions → 4 балла (45 / 10)
- 1 user_operation → CONWAY_PARTICIPANT (+100)
- 45 transactions → TRANSACTION_MILESTONE_10 (+0)

Итого: 104 балла
```

## Предварительные требования

1. **Linera CLI** установлен
2. **Rust toolchain** (1.75+)
3. **Node.js** (18+) для фронтенда
4. **Linera Indexer** запущен
5. Кошелек Linera с тестовыми токенами

## Шаг 1: Сборка контракта

```bash
cd /home/izhndvr/linera-protocol/examples/passport-nft

# Сборка Wasm binaries
cargo build --release --target wasm32-unknown-unknown

# Результат:
# - target/wasm32-unknown-unknown/release/passport_nft_contract.wasm
# - target/wasm32-unknown-unknown/release/passport_nft_service.wasm
```

## Шаг 2: Публикация приложения

```bash
# Публикация bytecode и создание приложения
linera publish-and-create \
  target/wasm32-unknown-unknown/release/passport_nft_contract.wasm \
  target/wasm32-unknown-unknown/release/passport_nft_service.wasm

# Сохраните выходные данные:
# Application ID: e476...
# Chain ID: e476...
```

## Шаг 3: Запуск Linera Service

```bash
# Запуск GraphQL service для приложения
linera service --port 8080

# GraphQL endpoint будет доступен по адресу:
# http://127.0.0.1:8080/chains/<CHAIN_ID>/applications/<APP_ID>
```

## Шаг 4: Запуск Linera Indexer

```bash
cd /home/izhndvr/linera-protocol/linera-indexer/example

# Запуск индексатора с Operations plugin
cargo run -- \
  --service-port 8080 \
  --port 8000 \
  --plugin operations

# Indexer API доступен по адресу:
# http://127.0.0.1:8000/operations
```

## Шаг 5: Конфигурация агента

Создайте файл `.env` или установите переменные окружения:

```bash
# В passport-nft-agent/.env
export PASSPORT_AGENT__WALLET_PATH="/path/to/wallet.json"
export PASSPORT_AGENT__APPLICATION_ID="e476..."
export PASSPORT_AGENT__OPERATION_CHAIN_ID="e476..."
export PASSPORT_AGENT__GRAPHQL_ENDPOINT="http://127.0.0.1:8080/chains/<CHAIN>/applications/<APP>"
export PASSPORT_AGENT__INDEXER_ENDPOINT="http://127.0.0.1:8000/operations"
export PASSPORT_AGENT__RULES_PATH="config/achievements.json"

# Опционально: OpenAI для AI-powered scoring
export PASSPORT_AGENT__OPENAI__API_KEY="sk-..."
export PASSPORT_AGENT__OPENAI__MODEL="gpt-4o-mini"
```

## Шаг 6: Сборка и запуск агента

```bash
cd /home/izhndvr/linera-protocol/examples/passport-nft-agent

# Сборка
cargo build --release

# Запуск в dry-run mode (без отправки в блокчейн)
cargo run --bin passport_oracle -- --dry-run --log-level debug

# Запуск с реальной отправкой обновлений
cargo run --bin passport_oracle -- --log-level info
```

### Выход агента (пример):

```
[INFO] Passport oracle configuration loaded
[INFO] Fetched 3 passports
[INFO] Processing passport: abc123...
[DEBUG] Score calculation breakdown:
  total_transactions=45
  base_score=4
  achievement_points=100
  total_score=104
[INFO] Passport evaluated: score=104, achievements=2
[INFO] Submitting update operation to blockchain
[INFO] Update submitted to blockchain
```

## Шаг 7: Запуск Web Frontend

```bash
cd /home/izhndvr/linera-protocol/examples/passport-nft/web-frontend

# Установка зависимостей
npm install

# Запуск dev сервера
npm run dev

# Или production build
npm run build
npm start
```

Откройте браузер:
```
http://localhost:3000/<CHAIN_ID>?app=<APP_ID>&owner=<OWNER_ADDRESS>
```

## Тестирование

### 1. Минт паспорта

Через UI:
1. Откройте веб-интерфейс
2. Нажмите "Mint Passport"
3. Подтвердите транзакцию

Через CLI:
```bash
linera --wallet <WALLET> \
  execute-operation \
  --application-id <APP_ID> \
  --operation-json '{
    "Mint": {
      "token_id": {"id": [1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]},
      "metadata_uri": "ipfs://...",
      "image_uri": "ipfs://...",
      "content_hash": "sha256..."
    }
  }'
```

### 2. Генерация активности

```bash
# Создайте несколько транзакций
for i in {1..15}; do
  linera transfer --amount 0.1 --to <RECIPIENT>
  sleep 1
done

# Вызовите контракт несколько раз
for i in {1..5}; do
  linera execute-operation --application-id <APP_ID> --operation-json '{...}'
  sleep 1
done
```

### 3. Запуск оракула

```bash
# Запустите агент для обновления скора
cargo run --bin passport_oracle

# Агент:
# 1. Прочитает все паспорта
# 2. Запросит активность каждого owner у индексатора
# 3. Вычислит новый скор
# 4. Отправит UpdateAchievements в блокчейн
```

### 4. Проверка результата

Через UI:
1. Нажмите "Refresh Passport"
2. Увидите обновленный скор и достижения

Через GraphQL:
```graphql
query {
  allPassports {
    tokenId { id }
    owner
    score
    achievements
  }
}
```

## Troubleshooting

### Агент не может подключиться к индексатору
```
Error: failed to fetch owner activity
```
**Решение:** Убедитесь, что Linera Indexer запущен на порту 8000

### Агент не может отправить транзакцию
```
Error: linera command failed
```
**Решение:**
- Проверьте, что `WALLET_PATH` указывает на правильный файл
- Убедитесь, что у owner есть токены для gas
- Проверьте, что `application_id` и `operation_chain_id` корректны

### Web frontend не видит паспорта
**Решение:**
- Убедитесь, что Linera Service запущен на порту 8080
- Проверьте правильность URL: `/chainId?app=appId&owner=ownerAddress`
- Откройте DevTools и проверьте Network вкладку

## Настройка скоринга

Измените `config/achievements.json`:

```json
{
  "scoring_rules": {
    "transactions_per_point": 10,  // Измените соотношение
    "base_multiplier": 1
  },
  "achievements": [
    {
      "code": "NEW_ACHIEVEMENT",
      "explanation": "Custom achievement",
      "points": 50,
      "condition": {
        "user_operation": {
          "min_count": 10
        }
      }
    }
  ]
}
```

## Production Checklist

- [ ] Контракт скомпилирован и опубликован
- [ ] Linera Service запущен и доступен
- [ ] Linera Indexer запущен и индексирует блоки
- [ ] Агент настроен с правильными credentials
- [ ] Web frontend развернут и доступен
- [ ] Протестирован полный flow: mint → activity → oracle update
- [ ] Мониторинг логов агента настроен
- [ ] Резервное копирование wallet.json

## Для Buildathon Demo

### Сценарий демонстрации:

1. **Показать Web UI** (2 мин)
   - Минт паспорта
   - Показать начальный скор = 0

2. **Создать активность** (3 мин)
   - Выполнить 15-20 транзакций
   - Вызвать несколько user operations

3. **Запустить Oracle** (2 мин)
   - Показать логи агента
   - Показать расчет скора

4. **Обновить UI** (1 мин)
   - Refresh passport
   - Показать новый скор и достижения

5. **Объяснить архитектуру** (2 мин)
   - Indexer собирает данные
   - Oracle анализирует активность
   - Обновляет on-chain паспорт

**Всего: 10 минут**

## Дополнительные возможности (будущее)

- [ ] AI-powered scoring через OpenAI
- [ ] Больше типов достижений
- [ ] Leaderboard
- [ ] NFT изображения генерируются динамически
- [ ] Экспорт паспорта в PDF
- [ ] Интеграция с другими приложениями Linera

## Контакты и поддержка

- GitHub: https://github.com/linera-io/linera-protocol
- Discord: Linera Official Server
- Docs: https://docs.linera.io

---

**Удачи на Buildathon! 🚀**
