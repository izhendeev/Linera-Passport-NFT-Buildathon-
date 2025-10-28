# 🛂 Passport NFT - Новый Простой Фронтенд

## 🚀 Быстрый старт (3 команды)

```bash
# 1. Запустите Linera Service (в отдельном терминале)
linera service --port 8080

# 2. Запустите Quick Score API (в отдельном терминале)
cd examples/passport-nft-agent
cargo run --bin quick_score_api --release

# 3. Запустите фронтенд
cd examples/passport-nft
bash start-frontend.sh
```

Скрипт автоматически откроет браузер с правильным URL!

---

## 📋 Что есть в новом фронтенде

### ✅ Функционал
- **Кнопка MINT** - создает NFT паспорт
- **Отображение NFT** с реальным скором из индексера
- **Кнопка UPDATE SCORE** - обновляет скор в реальном времени
- **Кнопка AI SCORE** - пока неактивна (для будущего)
- **Автообновление** - каждые 15 секунд

### 🎯 Особенности
- **Один HTML файл** - никаких зависимостей, npm, node_modules
- **Реальный скор** - получает данные напрямую из индексера
- **Достижения** - показывает все заработанные ачивки
- **Красивый UI** - градиенты, анимации, responsive дизайн

---

## 📁 Новые файлы

```
examples/passport-nft/
├── frontend.html              # ← Главный фронтенд (один файл!)
├── start-frontend.sh          # ← Скрипт автозапуска
├── test-score-api.sh          # ← Тест Quick Score API
├── README_NEW_FRONTEND.md     # ← Этот файл
└── FRONTEND_QUICK_START.md    # ← Подробная инструкция
```

---

## 🔧 Конфигурация

### Текущие настройки

- **Application ID**: `b139121af898c9bbb6dca05a7efde3ef396eeefe271650bb5659692613d4d463`
- **Chain ID**: `f7ebbdd68ad4fd2daf192575ad10c27bd7089d5e0a30facaf507f9bc22b9c6fe`
- **Admin**: `User:a2e5ed5897babe63f5220523e8502cd7093dac1972658ea29e0bac3c42aaff74`

### URL формат

```
http://localhost:3000/frontend.html?owner=YOUR_OWNER_ADDRESS
```

Остальные параметры (chainId, app) берутся из дефолтных значений в коде.

---

## 🎮 Как использовать

### 1. Минт паспорта
1. Откройте фронтенд с вашим owner address в URL
2. Нажмите **"MINT PASSPORT"**
3. Подождите 3 секунды - паспорт появится

### 2. Получить реальный скор
1. Сделайте активность в сети (транзакции, operations)
2. Нажмите **"UPDATE SCORE"**
3. Скор обновится из индексера в реальном времени

### 3. Проверить достижения
После UPDATE SCORE вы увидите все заработанные достижения:
- `early_adopter` (+50) - первые 1000 пользователей
- `active_user` (+25) - 10+ транзакций
- `power_user` (+75) - 50+ транзакций
- `whale` (+100) - 1000+ токенов
- `app_creator` (+150) - создал приложение
- `developer` (+100) - использовал 3+ приложения

---

## 🏗️ Архитектура

```
┌─────────────────┐
│   Browser       │
│  frontend.html  │
└────────┬────────┘
         │
         ├─────────────────────┐
         │                     │
         ▼                     ▼
┌─────────────────┐   ┌────────────────┐
│ Linera Service  │   │ Quick Score    │
│  GraphQL API    │   │     API        │
│   Port 8080     │   │   Port 8001    │
└────────┬────────┘   └───────┬────────┘
         │                    │
         ▼                    ▼
┌─────────────────────────────────────┐
│       Linera Blockchain             │
│  ┌──────────────┐  ┌──────────────┐ │
│  │ Passport NFT │  │   Indexer    │ │
│  │   Contract   │  │  (Port 8000) │ │
│  └──────────────┘  └──────────────┘ │
└─────────────────────────────────────┘
```

### Поток данных

1. **Mint**: Frontend → GraphQL → Contract → Blockchain
2. **Display**: Frontend → GraphQL → Contract (паспорт из блокчейна)
3. **Update Score**: Frontend → Quick Score API → Indexer → расчет → Frontend

---

## 🐛 Troubleshooting

### Проблема: "Failed to load passport"
```bash
# Проверьте что Linera Service запущен
ps aux | grep "linera service"

# Если нет, запустите:
linera service --port 8080
```

### Проблема: "Score API not available"
```bash
# Запустите Quick Score API
cd examples/passport-nft-agent
cargo run --bin quick_score_api --release

# Проверьте что API работает:
curl http://localhost:8001/quick-score?owner=User:a2e5ed58...
```

### Проблема: Скор всегда 0
```bash
# Сделайте транзакции для активности
linera transfer --amount 0.1 --to <RECIPIENT>
linera transfer --amount 0.1 --to <RECIPIENT>

# Обновите скор через кнопку UPDATE SCORE
```

---

## 📊 Система скоринга

### Базовый скор
- **1 балл за каждые 10 транзакций**
- Считаются: transfers + user operations + app creations

### Достижения
Правила в `examples/passport-nft-agent/config/achievements.json`

Изменить легко:
```bash
nano examples/passport-nft-agent/config/achievements.json
# Перезапустите Quick Score API после изменения
```

---

## 🎨 Кастомизация фронтенда

Откройте `frontend.html` и измените CSS в блоке `<style>`:

```css
/* Изменить главный градиент */
body {
    background: linear-gradient(135deg, #YOUR_COLOR1 0%, #YOUR_COLOR2 100%);
}

/* Изменить цвет кнопок */
button {
    background: linear-gradient(135deg, #YOUR_COLOR1 0%, #YOUR_COLOR2 100%);
}
```

---

## ⚡ Производительность

- **Размер**: 19 KB (один HTML файл)
- **Зависимости**: 0
- **Время загрузки**: < 100ms
- **API запросы**: 1 на UPDATE SCORE
- **Автообновление**: каждые 15 секунд (только если паспорт существует)

---

## 🔐 Безопасность

- Только чтение из блокчейна (GraphQL queries)
- Запись только через подписанные транзакции (mutations)
- Валидация owner address на клиенте
- Нет хранения приватных ключей

---

## 📚 Дополнительная документация

- [FRONTEND_QUICK_START.md](./FRONTEND_QUICK_START.md) - подробная инструкция
- [DEPLOYMENT.md](./DEPLOYMENT.md) - деплой контракта
- [../passport-nft-agent/README.md](../passport-nft-agent/README.md) - оракул агент

---

## 🎯 Roadmap

- [ ] AI Score интеграция (кнопка активна)
- [ ] История изменений скора
- [ ] Leaderboard топ пользователей
- [ ] NFT изображение генерируется динамически
- [ ] Экспорт паспорта в PDF
- [ ] Mobile app (PWA)

---

## 💡 FAQ

### Q: Зачем Quick Score API если есть контракт?
**A**: Контракт хранит только финальный скор. Quick Score API считает скор в реальном времени из индексера без записи в блокчейн. Это позволяет пользователю видеть актуальный скор мгновенно.

### Q: Как работает система достижений?
**A**: Quick Score API:
1. Запрашивает все транзакции из индексера
2. Применяет правила из `achievements.json`
3. Возвращает список заработанных достижений
4. Frontend отображает их

### Q: Можно ли минтить несколько паспортов?
**A**: Нет, контракт разрешает **только один паспорт на owner address** (soulbound NFT).

### Q: Где хранятся изображения NFT?
**A**: Сейчас используются placeholder IPFS URIs. В будущем можно интегрировать реальный IPFS или генерировать SVG динамически.

---

**Готово к использованию! 🎉**

Для помощи: открывайте issue в репозитории или пишите в Discord Linera.
