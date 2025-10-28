# Passport NFT - Quick Start Guide

## Простой фронтенд для Buildathon

Этот фронтенд - **один HTML файл** без зависимостей, который работает напрямую с вашим развернутым контрактом.

---

## ⚡ Быстрый старт

### 1. Запустите необходимые сервисы

#### a) Linera Service (GraphQL для блокчейна)
```bash
# В одном терминале
linera service --port 8080
```

#### b) Quick Score API (для реального скора из индексера)
```bash
# В другом терминале
cd /home/izhndvr/linera-protocol/examples/passport-nft-agent
cargo run --bin quick_score_api --release
```

Это запустит API на порту **8001** который будет возвращать реальный скор из индексера.

### 2. Откройте фронтенд

```bash
# Просто откройте в браузере:
firefox /home/izhndvr/linera-protocol/examples/passport-nft/frontend.html

# Или через простой HTTP сервер:
cd /home/izhndvr/linera-protocol/examples/passport-nft
python3 -m http.server 3000
# Затем откройте: http://localhost:3000/frontend.html
```

### 3. Добавьте параметры в URL

Фронтенд использует URL параметры для конфигурации:

```
http://localhost:3000/frontend.html?owner=YOUR_OWNER_ADDRESS

# Или с полными параметрами:
http://localhost:3000/frontend.html?owner=User:a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74&chainId=f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe&app=b139121af898c9bbb6dca05a7efde3ef396eeefe271650bb5659692613d4d463
```

**Параметры URL:**
- `owner` (обязательный) - адрес владельца (например: `User:a2e5ed58...`)
- `chainId` (опционально) - по умолчанию: `f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe`
- `app` (опционально) - по умолчанию: `b139121af898c9bbb6dca05a7efde3ef396eeefe271650bb5659692613d4d463`
- `port` (опционально) - порт Linera Service, по умолчанию: `8080`
- `scorePort` (опционально) - порт Quick Score API, по умолчанию: `8001`

---

## 🎯 Как работает фронтенд

### Кнопка "MINT PASSPORT"
1. Генерирует случайный token ID (16 байт)
2. Отправляет GraphQL mutation на контракт
3. Ждет подтверждения блокчейна (3 секунды)
4. Загружает созданный паспорт

### Кнопка "UPDATE SCORE"
1. Запрашивает **Quick Score API** (`/quick-score?owner=...`)
2. API обращается к **индексеру** и получает реальную активность пользователя
3. API вычисляет скор используя правила из `config/achievements.json`
4. Фронтенд отображает **реальный скор** и достижения

### Кнопка "AI SCORE" (отключена)
Будет использоваться для AI-анализа в будущем.

---

## 🔧 Как получить реальный скор из индексера

Quick Score API делает следующее:

1. **Получает все паспорты** из блокчейна через GraphQL
2. **Находит паспорт владельца** по адресу
3. **Запрашивает активность** у индексера:
   - Все транзакции (transfers)
   - Все user operations
   - Информацию о созданных приложениях
4. **Вычисляет скор** по правилам:
   - Базовый скор: 1 балл за каждые 10 транзакций
   - Достижения из `achievements.json`:
     - `early_adopter` (+50) - первые 1000 пользователей
     - `active_user` (+25) - 10+ транзакций
     - `power_user` (+75) - 50+ транзакций
     - `whale` (+100) - 1000+ токенов переведено
     - `app_creator` (+150) - создал приложение
     - `developer` (+100) - использовал 3+ приложения
5. **Возвращает JSON**:
```json
{
  "owner": "User:a2e5ed58...",
  "score": 175,
  "achievements": [
    {
      "code": "active_user",
      "explanation": "Made at least 10 transactions",
      "points": 25
    },
    {
      "code": "app_creator",
      "explanation": "Created at least one application on Linera",
      "points": 150
    }
  ],
  "method": "rule-based",
  "processing_time_ms": 342
}
```

---

## 📊 Как получить owner address

### Способ 1: Из wallet.json
```bash
# Показать все аккаунты в кошельке
linera wallet show

# Или напрямую из файла
cat ~/.config/linera/wallet.json | grep -A2 '"owner"'
```

Вы увидите что-то вроде:
```
"owner": "User:a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74"
```

### Способ 2: Через переменные окружения
Если вы использовали `deploy.sh` или `publish.sh`, owner может быть в `.env.deployment`:

```bash
cat /home/izhndvr/linera-protocol/examples/passport-nft/.env.deployment
```

---

## 🐛 Troubleshooting

### Ошибка: "Failed to load passport: fetch failed"
**Проблема:** Linera Service не запущен или не доступен

**Решение:**
```bash
# Проверьте что сервис запущен
ps aux | grep "linera service"

# Если нет, запустите:
linera service --port 8080
```

### Ошибка: "Score API not available"
**Проблема:** Quick Score API не запущен

**Решение:**
```bash
cd /home/izhndvr/linera-protocol/examples/passport-nft-agent
cargo run --bin quick_score_api --release
```

API должен показать:
```
Quick Score API listening on http://127.0.0.1:8001
Example: http://127.0.0.1:8001/quick-score?owner=0x...
```

### Скор всегда 0
**Проблема:** Нет активности в сети или индексер не видит транзакции

**Решение:**
1. Сделайте несколько транзакций:
```bash
# Простые переводы
linera transfer --amount 0.1 --to <RECIPIENT>
linera transfer --amount 0.1 --to <RECIPIENT>
linera transfer --amount 0.1 --to <RECIPIENT>
```

2. Обновите скор через кнопку "UPDATE SCORE"

3. Проверьте что индексер работает:
```bash
# Запрос к индексеру напрямую
curl http://localhost:8000/operations
```

### Минт не работает
**Проблема:** Недостаточно токенов для gas или паспорт уже существует

**Решение:**
1. Проверьте баланс:
```bash
linera wallet show
```

2. Если паспорт уже существует (один паспорт на адрес):
   - Используйте другой owner address
   - Или просто используйте кнопку "UPDATE SCORE"

---

## 🎨 Кастомизация

### Изменить внешний вид
Все стили в `<style>` блоке в `frontend.html`. Можете менять цвета, градиенты, шрифты.

### Изменить правила скоринга
Отредактируйте файл:
```bash
nano /home/izhndvr/linera-protocol/examples/passport-nft-agent/config/achievements.json
```

Затем перезапустите Quick Score API.

### Добавить новые достижения
В `achievements.json` добавьте новое правило:
```json
{
  "code": "super_user",
  "explanation": "Made 100+ transactions",
  "points": 200,
  "condition": {
    "total_transactions": {
      "min_count": 100
    }
  }
}
```

---

## 📝 Файлы

```
examples/passport-nft/
├── frontend.html              # ← НОВЫЙ ФРОНТЕНД (один файл, без зависимостей)
├── FRONTEND_QUICK_START.md    # ← Эта инструкция
├── .env.deployment            # Конфигурация деплоя (APP_ID, CHAIN_ID, ADMIN)
├── src/                       # Исходники контракта
└── web-frontend/              # Старый Next.js фронтенд (не используется)

examples/passport-nft-agent/
├── src/bin/quick_score_api.rs # ← Quick Score API (получает реальный скор)
├── src/bin/passport_oracle.rs # Полный оракул (пишет в блокчейн)
└── config/achievements.json   # Правила скоринга
```

---

## 🚀 Полный пример workflow

```bash
# Терминал 1: Запуск Linera Service
linera service --port 8080

# Терминал 2: Запуск Quick Score API
cd examples/passport-nft-agent
cargo run --bin quick_score_api --release

# Терминал 3: Запуск HTTP сервера для фронтенда
cd examples/passport-nft
python3 -m http.server 3000

# Откройте браузер:
firefox "http://localhost:3000/frontend.html?owner=User:a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74"

# В браузере:
# 1. Нажмите "MINT PASSPORT" - создастся NFT
# 2. Сделайте транзакции в сети Linera
# 3. Нажмите "UPDATE SCORE" - получите реальный скор из индексера
```

---

## ✅ Что работает

- ✅ Минт паспорта через GraphQL
- ✅ Отображение NFT с token ID, owner, score
- ✅ **Реальный скор из индексера** через Quick Score API
- ✅ Достижения на основе реальной активности
- ✅ Автообновление каждые 15 секунд
- ✅ Responsive дизайн
- ✅ Обработка ошибок

## 🔮 Roadmap

- ⏳ AI Score кнопка (интеграция с OpenAI)
- ⏳ Визуализация истории скора
- ⏳ Leaderboard
- ⏳ NFT изображения

---

**Удачи на Buildathon! 🎉**