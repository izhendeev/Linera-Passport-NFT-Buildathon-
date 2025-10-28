# 🎉 НОВЫЙ ФРОНТЕНД ГОТОВ!

## ✅ Что создано

### 1. Главный фронтенд
**Файл**: `frontend.html` (19 KB, один файл без зависимостей)

**Функции**:
- ✅ Кнопка **MINT** - создает паспорт NFT
- ✅ Отображение паспорта с:
  - Token ID
  - Owner address
  - **Реальный скор из индексера** (через Quick Score API)
  - Список достижений с описанием и баллами
  - Дата создания
- ✅ Кнопка **UPDATE SCORE** - обновляет скор в реальном времени
- ✅ Кнопка **AI SCORE** - неактивна (для будущего)
- ✅ Автообновление каждые 15 секунд
- ✅ Красивый UI с градиентами и анимациями
- ✅ Responsive дизайн

### 2. Скрипт автозапуска
**Файл**: `start-frontend.sh`

**Что делает**:
1. Проверяет что Linera Service запущен (8080)
2. Запускает Quick Score API если не запущен (8001)
3. Запускает HTTP сервер для фронтенда (3000)
4. Генерирует правильный URL с параметрами
5. Автоматически открывает браузер

**Использование**:
```bash
cd examples/passport-nft
bash start-frontend.sh
```

### 3. Скрипт тестирования
**Файл**: `test-score-api.sh`

**Что делает**:
- Тестирует работу Quick Score API
- Показывает реальный скор пользователя
- Форматирует JSON с jq (если доступен)

**Использование**:
```bash
cd examples/passport-nft
bash test-score-api.sh
```

### 4. Документация
**Файлы**:
- `README_NEW_FRONTEND.md` - главный README с FAQ и архитектурой
- `FRONTEND_QUICK_START.md` - подробная инструкция по запуску
- `НОВЫЙ_ФРОНТ_SUMMARY.md` - этот файл (краткое резюме)

---

## 🚀 БЫСТРЫЙ СТАРТ

### Вариант 1: Автоматический (рекомендуется)

```bash
# Терминал 1: Запустите Linera Service
linera service --port 8080

# Терминал 2: Запустите всё остальное
cd examples/passport-nft
bash start-frontend.sh
```

Скрипт сам:
- Запустит Quick Score API
- Запустит HTTP сервер
- Откроет браузер с правильным URL

### Вариант 2: Ручной

```bash
# Терминал 1
linera service --port 8080

# Терминал 2
cd examples/passport-nft-agent
cargo run --bin quick_score_api --release

# Терминал 3
cd examples/passport-nft
python3 -m http.server 3000

# Откройте браузер:
firefox "http://localhost:3000/frontend.html?owner=YOUR_OWNER_ADDRESS"
```

---

## 🎯 КАК ЭТО РАБОТАЕТ

### Минт паспорта
```
User нажимает MINT
    ↓
Frontend генерирует token ID
    ↓
GraphQL mutation → Linera Service
    ↓
Contract создает паспорт
    ↓
Паспорт записывается в blockchain
    ↓
Frontend загружает паспорт (через 3 сек)
```

### Обновление скора
```
User нажимает UPDATE SCORE
    ↓
Frontend → Quick Score API (http://localhost:8001/quick-score?owner=...)
    ↓
Quick Score API:
  1. Запрашивает паспорт из GraphQL
  2. Запрашивает активность из Indexer
  3. Применяет правила из achievements.json
  4. Считает скор = базовый + достижения
    ↓
API возвращает { score: 150, achievements: [...] }
    ↓
Frontend отображает реальный скор и достижения
```

### Система скоринга

**Базовый скор**:
- 1 балл за каждые 10 транзакций

**Достижения** (из `config/achievements.json`):
- `early_adopter` = +50 (первые 1000 пользователей)
- `active_user` = +25 (10+ транзакций)
- `power_user` = +75 (50+ транзакций)
- `whale` = +100 (1000+ токенов переведено)
- `app_creator` = +150 (создал приложение)
- `developer` = +100 (использовал 3+ приложения)

**Пример расчета**:
```
Пользователь сделал:
- 35 транзакций → 3 балла (35 ÷ 10 = 3)
- 10+ транзакций → active_user (+25)
- 35 транзакций → power_user НЕ заработан (нужно 50+)

Итого: 3 + 25 = 28 баллов
```

---

## 📁 ФАЙЛОВАЯ СТРУКТУРА

```
examples/passport-nft/
├── frontend.html                    ← НОВЫЙ ФРОНТЕНД ⭐
├── start-frontend.sh                ← Скрипт автозапуска ⭐
├── test-score-api.sh                ← Тест Quick Score API ⭐
├── README_NEW_FRONTEND.md           ← Главный README ⭐
├── FRONTEND_QUICK_START.md          ← Подробная инструкция ⭐
├── НОВЫЙ_ФРОНТ_SUMMARY.md           ← Этот файл ⭐
│
├── .env.deployment                  # Конфиг (APP_ID обновлен на новый!)
├── src/                             # Исходники контракта
│   ├── contract.rs
│   ├── service.rs
│   ├── state.rs
│   └── token.rs
│
└── web-frontend/                    # Старый Next.js фронт (не используется)
    └── ...

examples/passport-nft-agent/
├── src/bin/
│   ├── quick_score_api.rs           # Quick Score API (используется фронтом) ✅
│   ├── passport_oracle.rs           # Полный оракул (пишет в blockchain)
│   └── mint_api.rs                  # Demo mint API
│
└── config/
    └── achievements.json            # Правила скоринга ✅
```

---

## 🔧 КОНФИГУРАЦИЯ

### Обновленные значения

```bash
APPLICATION_ID=b139121af898c9bbb6dca05a7efde3ef396eeefe271650bb5659692613d4d463  # ✅ НОВЫЙ!
CHAIN_ID=f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe
ADMIN_ACCOUNT=User:a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74
```

APP_ID обновлен во всех файлах:
- ✅ `frontend.html` (дефолтное значение)
- ✅ `FRONTEND_QUICK_START.md` (примеры)
- ✅ `start-frontend.sh` (скрипт автозапуска)

### Порты

- **8080** - Linera Service (GraphQL для контракта)
- **8000** - Linera Indexer (используется Quick Score API)
- **8001** - Quick Score API (используется фронтом)
- **3000** - HTTP сервер для фронтенда

---

## ✅ ЧТО РАБОТАЕТ

1. ✅ **Минт паспорта** - через GraphQL mutation
2. ✅ **Отображение NFT** - owner, token ID, created date
3. ✅ **Реальный скор из индексера** - через Quick Score API
4. ✅ **Достижения с баллами** - подробное описание каждого
5. ✅ **Обновление в реальном времени** - кнопка UPDATE SCORE
6. ✅ **Автообновление** - каждые 15 секунд
7. ✅ **Красивый UI** - градиенты, анимации, responsive
8. ✅ **Обработка ошибок** - понятные сообщения об ошибках
9. ✅ **Никаких зависимостей** - один HTML файл, 19 KB

---

## 🎨 ДИЗАЙН

### Цветовая схема
- **Фон**: Градиент purple-blue (#1e3c72 → #2a5298 → #7e22ce)
- **Карточки**: Темно-серый (#1a1a1a, #2a2a2a)
- **Кнопки**: Purple gradient (#7e22ce → #9333ea)
- **Скор**: Purple gradient background с крупным числом
- **Текст**: Белый (#fff) + серый (#888, #aaa)

### UI компоненты
- Passport карточка с разделами
- Большой скор в отдельном блоке с градиентом
- Список достижений с прокруткой
- Три кнопки: Mint, Update Score, AI Score
- Статус бар внизу
- Информация о конфигурации вверху

---

## 🐛 TROUBLESHOOTING

### Если фронт не загружается
```bash
# Проверьте что HTTP сервер запущен
ps aux | grep "http.server"

# Если нет:
cd examples/passport-nft
python3 -m http.server 3000
```

### Если не работает Mint
```bash
# Проверьте Linera Service
curl http://localhost:8080

# Должно вернуть HTML страницу GraphQL playground
```

### Если скор всегда 0
```bash
# Проверьте Quick Score API
curl "http://localhost:8001/quick-score?owner=YOUR_OWNER"

# Должно вернуть JSON с score и achievements
```

### Если Quick Score API не запущен
```bash
cd examples/passport-nft-agent
cargo run --bin quick_score_api --release

# Должно показать:
# Quick Score API listening on http://127.0.0.1:8001
```

---

## 📊 ТЕСТИРОВАНИЕ

### Полный workflow

```bash
# 1. Запустите сервисы
linera service --port 8080  # Terminal 1
bash start-frontend.sh      # Terminal 2

# 2. Откройте фронт в браузере
# (автоматически откроется или вручную)

# 3. Минтим паспорт
# Нажмите кнопку "MINT PASSPORT"
# Ждем 3 секунды → паспорт появился!

# 4. Создаем активность
linera transfer --amount 0.1 --to <RECIPIENT>
linera transfer --amount 0.1 --to <RECIPIENT>
# ... повторите 10+ раз

# 5. Обновляем скор
# Нажмите "UPDATE SCORE"
# Скор обновится!

# Пример результата:
# Score: 26 (1 базовый + 25 за active_user)
# Achievement: active_user: Made at least 10 transactions (+25 points)
```

### Тест Quick Score API

```bash
cd examples/passport-nft
bash test-score-api.sh

# Покажет:
# Request: http://localhost:8001/quick-score?owner=...
# Response: { "score": 26, "achievements": [...], ... }
```

---

## 🚀 ДЕПЛОЙ ДЛЯ BUILDATHON

### Подготовка к демонстрации

1. **Запустите все сервисы заранее**:
```bash
# Terminal 1
linera service --port 8080

# Terminal 2
cd examples/passport-nft-agent
cargo run --bin quick_score_api --release

# Terminal 3
cd examples/passport-nft
python3 -m http.server 3000
```

2. **Подготовьте тестовый аккаунт**:
```bash
# Создайте тестовый кошелек с активностью
# Сделайте 20+ транзакций заранее
# Минтите паспорт заранее
```

3. **Откройте фронт в браузере**:
```
http://localhost:3000/frontend.html?owner=YOUR_TEST_OWNER
```

4. **Проверьте все работает**:
- ✅ Паспорт отображается
- ✅ Скор > 0
- ✅ Есть достижения
- ✅ UPDATE SCORE работает

### Сценарий демонстрации (5 минут)

**Минута 1-2**: Показать фронтенд
- "Это простой фронтенд для Passport NFT"
- "Один HTML файл, никаких зависимостей"
- Показать UI, скор, достижения

**Минута 2-3**: Объяснить функционал
- "Пользователь может минтить паспорт NFT"
- "Скор считается из реальной активности в сети"
- "Система достижений мотивирует пользователей"

**Минута 3-4**: Live update
- Сделать 2-3 транзакции в реальном времени
- Нажать UPDATE SCORE
- Показать как скор изменился

**Минута 4-5**: Архитектура
- "Quick Score API получает данные из индексера"
- "Применяет правила скоринга"
- "Возвращает реальный скор без записи в блокчейн"
- "Это позволяет видеть скор мгновенно"

---

## 🎯 ГОТОВО К ИСПОЛЬЗОВАНИЮ!

### Что нужно сделать СЕЙЧАС:

1. ✅ **Запустите сервисы**:
```bash
linera service --port 8080              # Terminal 1
cd examples/passport-nft
bash start-frontend.sh                   # Terminal 2
```

2. ✅ **Откройте фронт** - автоматически откроется или вручную:
```
http://localhost:3000/frontend.html?owner=YOUR_OWNER
```

3. ✅ **Минтите паспорт** - нажмите MINT

4. ✅ **Сделайте активность** - несколько транзакций

5. ✅ **Обновите скор** - нажмите UPDATE SCORE

6. ✅ **Готово!** 🎉

---

## 💡 ПОЛЕЗНЫЕ КОМАНДЫ

```bash
# Узнать свой owner address
linera wallet show | grep owner

# Проверить баланс
linera wallet show

# Сделать транзакцию
linera transfer --amount 0.1 --to <RECIPIENT>

# Посмотреть все паспорта
curl -X POST http://localhost:8080/chains/$CHAIN_ID/applications/$APP_ID \
  -H "Content-Type: application/json" \
  -d '{"query": "{ allPassports { owner score achievements } }"}'

# Протестировать Quick Score API
curl "http://localhost:8001/quick-score?owner=$OWNER" | jq .
```

---

## 📞 ПОДДЕРЖКА

Если что-то не работает:
1. Проверьте что все сервисы запущены (8080, 8001, 3000)
2. Посмотрите логи в консоли браузера (F12)
3. Проверьте owner address в URL
4. Прочитайте [FRONTEND_QUICK_START.md](./FRONTEND_QUICK_START.md) для подробностей

---

**Успехов на Buildathon! 🚀🎉**
