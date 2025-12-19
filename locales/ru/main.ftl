# Russian translations for AI Todolist Bot

# === Welcome & Start ===
welcome = <b>{ $name }</b>, добро пожаловать!

    Я твой AI-помощник по задачам.
    Текст или голос — я пойму.

    <code>Позвонить маме завтра в 17:00</code>
    Попробуй ↑

welcome-trial = 🎁 У тебя <b>7 дней бесплатного периода</b>!
welcome-referral-bonus = 🎉 +7 дней бонус по реферальной ссылке!

# === Help ===
help-text = <b>Доступные команды:</b>

    /tasks — Список задач
    /today — Задачи на сегодня
    /settings — Настройки и подписка
    /invite — Пригласи друзей, получи +7 дней
    /support — Связаться с поддержкой

    Просто отправь сообщение, чтобы создать задачу!

# === Tasks ===
task-created = ✅ Задача создана: <b>{ $title }</b>
task-created-with-reminder = ✅ Задача создана: <b>{ $title }</b>
    ⏰ Напоминание: { $reminder }
task-done = ✅ Выполнено: <b>{ $title }</b>
task-deleted = 🗑 Удалено: { $title }
task-updated = ✏️ Обновлено: <b>{ $title }</b>

tasks-empty = 📋 Задач пока нет!

    Отправь мне сообщение, чтобы создать.
tasks-header = 📋 Твои задачи ({ $count }):
tasks-today-empty = 📅 На сегодня задач нет.

    Наслаждайся свободным временем! 🌴
tasks-today-header = 📅 Задачи на сегодня ({ $count }):

task-status-pending = ⏳
task-status-in-progress = 🔄
task-status-done = ✅

# === Task Actions ===
task-snooze-1h = Отложено на 1 час
task-snooze-tomorrow = Отложено на завтра
task-reminder-snoozed = ⏰ Напоминание отложено: { $title }

# === Reminders ===
reminder-title = ⏰ <b>Напоминание</b>
reminder-task = { $title }
reminder-due = 📅 Срок: { $due }
reminder-urgent = 🔴 Срочно!
reminder-soon = 🟡 Через 30 минут
reminder-normal = ⏰ Напоминание

# === Morning Brief ===
brief-title = ☀️ <b>Доброе утро!</b>
brief-greeting-morning = 👋 Доброе утро, { $name }!
brief-greeting-afternoon = 👋 Добрый день, { $name }!
brief-greeting-evening = 👋 Добрый вечер, { $name }!
brief-tasks-count = У тебя <b>{ $count }</b> { $count ->
    [one] задача
    [few] задачи
    *[other] задач
} на сегодня:
brief-today-header = 📅 Задачи на сегодня ({ $count }):
brief-no-tasks = 📅 На сегодня задач нет.
brief-other-pending = 📋 Ещё { $count } { $count ->
    [one] задача
    [few] задачи
    *[other] задач
} в ожидании
brief-outro = Продуктивного дня! 🚀
brief-tip = 💡 Отправь голосовое сообщение, чтобы быстро добавить задачу!
btn-view-tasks = 📋 Задачи

# === Weekly Review ===
weekly-title = 📊 <b>Итоги недели</b>
weekly-greeting = Привет, { $name }!
weekly-this-week = На этой неделе:
weekly-completed = ✅ Выполнено: <b>{ $count }</b> { $count ->
    [one] задача
    [few] задачи
    *[other] задач
}
weekly-created = ➕ Добавлено: <b>{ $count }</b>
weekly-pending = 📋 В ожидании: <b>{ $count }</b>
weekly-great-job = 🎉 Отличная работа на этой неделе!
weekly-keep-going = 💪 Так держать!
weekly-celebrate-good = Хороший прогресс! 👍
weekly-celebrate-great = Отличная неделя! 🔥
weekly-celebrate-productive = Продуктивная неделя! ⚡
weekly-celebrate-incredible = Невероятная продуктивность! 🏆
weekly-stale-warning = ⚠️ { $count } { $count ->
    [one] задача устарела
    [few] задачи устарели
    *[other] задач устарели
} (7+ дней). Пора проверить?
weekly-outro = Отличной недели! 🚀
btn-review-stale = 📋 Проверить
btn-view-all = 📊 Все задачи

# === Settings ===
settings-title = ⚙️ <b>Настройки</b>
settings-status = 📊 Статус: { $status }
settings-timezone = 🌍 Часовой пояс: { $tz }
settings-brief-time = ⏰ Утренний дайджест: { $time }
settings-language = 🌐 Язык: { $lang }
settings-full = ⚙️ <b>Настройки</b>

    📊 Статус: { $status }
    🌍 Часовой пояс: { $tz }
    ⏰ Утренний дайджест: { $time }
    🌐 Язык: { $lang }

settings-timezone-title = 🌍 <b>Выбери часовой пояс:</b>
settings-brief-title = ⏰ <b>Время утреннего дайджеста:</b>
settings-language-title = 🌐 <b>Выбери язык:</b>

settings-updated = ✅ Настройки обновлены!
settings-timezone-updated = ✅ Часовой пояс установлен: { $tz }
settings-brief-updated = ✅ Утренний дайджест установлен на { $time }
settings-language-updated = ✅ Язык изменён на { $lang }

# === Subscription ===
subscription-trial = 🎁 Пробный период ({ $days } { $days ->
    [one] день
    [few] дня
    *[other] дней
})
subscription-active = ✅ Активна ({ $days } { $days ->
    [one] день
    [few] дня
    *[other] дней
})
subscription-trial-warning = ⚠️ Пробный период заканчивается через { $days } { $days ->
    [one] день
    [few] дня
    *[other] дней
}
subscription-expired = ❌ Истекла

subscription-expired-message = ⏰ Пробный период закончился!

    Чтобы продолжить использовать AI Todolist, оформи подписку всего за <b>⭐50 Stars/месяц</b>.

    ✅ Безлимитные задачи
    ✅ Голосовые сообщения
    ✅ Умные напоминания
    ✅ Утренние дайджесты

subscription-success = 🎉 Спасибо за подписку!

    ✅ Твоя { $type } подписка активна до { $expires }.

    Пользуйся AI Todolist! 🚀

# === Referrals ===
invite-title = 🎁 <b>Пригласи друзей</b>
invite-description = Поделись ссылкой и получи <b>+7 дней</b> за каждого друга!
invite-link = Твоя реферальная ссылка:
invite-stats = 👥 Приглашено: <b>{ $count }</b>
    🎁 Бонусных дней: <b>{ $bonus }</b>
invite-share = 📲 Поделиться:
invite-full = 🎁 <b>Пригласи друзей</b>

    Твоя ссылка:
    <code>https://t.me/{ $bot }?start={ $code }</code>

    ✨ Вы оба получите +7 дней бесплатно!

    👥 Приглашено: { $count } · Бонус: { $bonus } дней

# === Support ===
support-prompt = 📮 <b>Связаться с поддержкой</b>

    Отправь сообщение ниже, и мы ответим в ближайшее время.
support-sent = ✅ Сообщение отправлено в поддержку. Мы скоро ответим!

# === Stale Tasks ===
stale-warning = ⚠️ { $count } { $count ->
    [one] задача не обновлялась
    [few] задачи не обновлялись
    *[other] задач не обновлялись
} более 7 дней.

    Стоит проверить или завершить их!

# === Duplicate Detection ===
duplicate-warning = ⚠️ Похожая задача уже есть:
    <i>{ $title }</i>

    Всё равно создать?

# === Celebration ===
celebration-streak = 🔥 <b>{ $count } задач подряд!</b> Так держать!
celebration-milestone = 🏆 <b>Веха!</b> Ты выполнил { $count } задач!
celebrate-first = 🎯 Первая задача сделана!
celebrate-keep-going = 👍 Так держать!
celebrate-on-fire = 🔥 Ты в ударе!
celebrate-unstoppable = 💪 Неостановим!

# === Task Completion ===
task-completed-stats = ✅ <b>{ $title }</b>

    { $celebration }
    📊 { $done }/{ $total } сделано сегодня
task-next = 📝 Далее: <b>{ $title }</b>{ $due }
task-all-done = 🎉 Всё сделано! Ты выполнил { $count } { $count ->
    [one] задачу
    [few] задачи
    *[other] задач
} сегодня!
task-delete-confirm = 🗑 Удалить <b>{ $title }</b>?
task-snoozed = ⏰ Отложено: <b>{ $title }</b>

    Напомню { $when }.
task-deleted-msg = 🗑 Удалено: { $title }
task-cancelled = Отменено
task-added = ✅ Добавлено!

    📝 { $title }{ $due }
task-create-failed = ❌ Не удалось создать задачу
session-expired = Сессия истекла

# === Stale Tasks ===
stale-no-tasks = ✅ Нет устаревших задач!
stale-reviewing = 📋 Проверяем { $count } { $count ->
    [one] устаревшую задачу
    [few] устаревшие задачи
    *[other] устаревших задач
}...
stale-task-item = 🕐 { $title }

    📅 Обновлено: { $updated }
stale-kept-all = ✅ Сохранено { $count } { $count ->
    [one] задача
    [few] задачи
    *[other] задач
}. Они не будут отмечены как устаревшие ещё 7 дней.
stale-kept-one = ✅ Сохранено: { $title }
btn-keep = ✅ Оставить
btn-review = 📋 Обзор
btn-keep-all = ✓ Оставить все

# === Timezone ===
tz-select-title = 🌍 Выбери часовой пояс:

    📍 Автоопределение по геолокации
    🏙 Ввести город вручную
tz-auto-prompt = 📍 Нажми кнопку ниже, чтобы поделиться геолокацией.

    Я определю часовой пояс автоматически.
tz-city-prompt = 🏙 Введи название города:

    Примеры: Москва, Киев, Минск, Алматы
tz-updated = ✅ Часовой пояс установлен: { $tz }
btn-auto-detect = 📍 Автоопределение
btn-type-city = 🏙 Ввести город

# === Brief Time ===
brief-select-title = ⏰ Выбери время утреннего дайджеста:
brief-updated = ✅ Время дайджеста установлено: { $time }

# === Language ===
lang-select-title = 🌐 Select language / Выберите язык:
lang-updated-en = ✅ Language changed to English
lang-updated-ru = ✅ Язык изменён на Русский

# === Voice ===
voice-processing = 🎤 Обрабатываю голосовое сообщение...
voice-transcribed = 📝 Распознано: <i>{ $text }</i>

# === Draft ===
draft-created = 📝 <b>Черновик сообщения:</b>

    { $text }

    <i>Скопируй и отправь, когда будешь готов!</i>

# === Vague Task ===
vague-suggestion = 💡 Эта задача кажется размытой. Хочешь разбить её?

    Предложения:
    { $suggestions }

# === Tags ===
tag-work = 💼 Работа
tag-personal = 🏠 Личное
tag-shopping = 🛒 Покупки
tag-health = 🏥 Здоровье
tag-finance = 💰 Финансы
tag-other = 📌 Другое

# === Buttons ===
btn-settings = ⚙️ Настройки
btn-done = ✅ Готово
btn-delete = 🗑 Удалить
btn-edit = ✏️ Изменить
btn-snooze-1h = ⏰ 1ч
btn-snooze-tomorrow = 📅 Завтра
btn-back = ← Назад
btn-timezone = 🌍 Часовой пояс
btn-brief-time = ⏰ Время дайджеста
btn-language = 🌐 Язык
btn-invite = 🎁 Пригласить друзей
btn-subscribe = ⭐ Подписаться
btn-create-anyway = ✅ Создать всё равно
btn-cancel = ❌ Отмена
btn-yes = ✅ Да
btn-no = ❌ Нет
btn-do-it = ✅ Сделать
btn-all-tasks = 📋 Все задачи
btn-yes-delete = 🗑 Да, удалить
snooze-1h = 1 час
snooze-tomorrow = завтра
snooze-later = позже

# === Errors ===
error-generic = ❌ Что-то пошло не так. Попробуй ещё раз.
error-not-found = ❌ Задача не найдена.
error-subscription-required = ⭐ Для этой функции нужна подписка.
error-start-first = Сначала нажми /start
error-use-buttons = ⚠️ Пожалуйста, используй кнопки выше.
error-task-create = ❌ Не удалось создать задачу

    Что-то пошло не так.

    💡 Попробуй: "Купить молоко завтра в 17:00"
error-voice-failed = 🎤 Не удалось распознать голос

    Аудио недостаточно чёткое.

    💡 Попробуй говорить ближе к микрофону или напиши текстом
error-voice-requires-ai = ❌ Для голосовых сообщений нужен AI-сервис
error-ai-required = ❌ Требуется AI-сервис
error-edit-failed = ❌ Не удалось понять инструкцию

    💡 Попробуй: "изменить время на 17:00" или "заменить Ивана на Петра"
error-timezone-failed = ❌ Не удалось определить часовой пояс

    💡 Попробуй крупный город: "Москва" или "Киев"
error-reminder-time-failed = ❌ Не удалось понять время

    💡 Попробуй: "завтра в 9" или "через 2 часа"

# === Voice ===
voice-processing = 🎤 Обрабатываю голос...

# === Edit ===
edit-title = ✏️ Редактирование:

    📝 { $title }{ $due }

    Что изменить?
edit-send-title = 📝 Сейчас: { $title }

    Отправь новый заголовок:
edit-preview = ✏️ Предпросмотр:

    📝 { $old_title } → { $new_title }{ $due_change }

    Применить изменения?
edit-cancelled = ❌ Редактирование отменено
edit-applied = ✅ Обновлено!

    📝 { $title }{ $due }
btn-apply = ✅ Применить
btn-edit-title = 📝 Заголовок
btn-edit-date = 📅 Дата

# === Date Selection ===
date-select-title = 📅 Выбери новую дату:
btn-date-today = 📅 Сегодня
btn-date-tomorrow = 📅 Завтра
btn-date-next-week = 📅 Через неделю
btn-date-remove = 🚫 Убрать дату
date-updated = ✅ Дата обновлена

# === Reminder Selection ===
remind-select-title = ⏰ Напоминание для:

    📝 { $title }

    { $current }

    ✍️ Своё время: текст или 🎤 голос
remind-custom-prompt = ⏰ Когда напомнить о:
    📝 { $title }

    Отправь время (текст или 🎤 голос):
    • "завтра в 15:00"
    • "через 2 часа"
    • "в понедельник утром"
btn-remind-30min = ⏰ 30 мин
btn-remind-1h = ⏰ 1 час
btn-remind-3h = ⏰ 3 часа
btn-remind-tomorrow = ⏰ Завтра 9:00
btn-remind-custom = ✍️ Своё
btn-remind-remove = 🚫 Убрать
remind-removed = 🔕 Напоминание убрано
remind-set = ⏰ Напоминание: { $when }
no-reminder = Напоминание не установлено

# === Stale Tasks ===
btn-stale-review = 📋 Обзор
btn-stale-keep-all = ✓ Оставить все

# === Subscription Expired ===
subscription-expired-full = ⏰ Пробный период закончился!

    Чтобы продолжить использовать AI Todolist, оформи подписку:

    ⭐ 1 месяц — 250 Stars (~$5)
    ⭐ 3 месяца — 600 Stars (~$12)
    ⭐ 12 месяцев — 2000 Stars (~$40)

    Используй /settings → Подписаться

# === Draft ===
draft-message = ✉️ Черновик для { $recipient }:

    { $text }

    💡 Скопируй и отправь!

# === Clarify ===
clarify-prompt = 🤔 { $question }

    Выбери конкретное действие:
btn-create-as-is = 📝 Создать как есть

# === AI Commands ===
ai-tasks-empty = 📋 Задач нет!

    💡 Отправь задачу: "Позвонить маме завтра"
ai-tasks-more = ...и ещё { $count }
ai-today-empty = 📅 На сегодня задач нет!

    💡 Добавь: "Встреча в 15:00"
ai-today-header = 📅 Сегодня ({ $count } { $count ->
    [one] задача
    [few] задачи
    *[other] задач
}):
ai-unknown-command = 🤖 Неизвестная команда

# === Voice Transcription Responses ===
voice-transcribed = 🎤 "{ $text }"
voice-transcribed-added = 🎤 "{ $text }"

    ✅ Добавлено!

    📝 { $title }{ $due }
voice-transcribed-error = 🎤 "{ $text }"

    ❌ Не удалось понять

    💡 Попробуй: "изменить время на 17:00"
voice-transcribed-reminder-set = 🎤 "{ $text }"

    ⏰ Напоминание установлено!

    📝 { $title }
    🔔 { $reminder }
voice-transcribed-reminder-failed = 🎤 "{ $text }"

    ❌ Не удалось понять время

    💡 Попробуй: "завтра в 9" или "через 2 часа"
voice-transcribed-timezone-set = 🎤 "{ $text }"

    ✅ Часовой пояс: { $timezone }
voice-transcribed-timezone-failed = 🎤 "{ $text }"

    ❌ Не удалось определить часовой пояс

    💡 Попробуй название крупного города
voice-transcribed-support-sent = 🎤 "{ $text }"

    ✅ Сообщение отправлено в поддержку!
voice-transcribed-support-failed = ❌ Не удалось отправить. Попробуй ещё раз.
voice-transcribed-draft = 🎤 "{ $text }"

    ✉️ Черновик для { $recipient }:

    { $draft }

    💡 Скопируй и отправь!
voice-transcribed-clarify = 🎤 "{ $text }"

    🤔 { $question }

    Выбери конкретное действие:
voice-transcribed-unknown = 🎤 "{ $text }"

    🤖 { $reason }
voice-task-create-failed = ❌ Не удалось создать задачу

    Что-то пошло не так.

    💡 Попробуй написать текстом

# === Support Flow ===
support-sent-success = ✅ Сообщение отправлено!

    Ответим в течение 24 часов.
support-failed = ❌ Не удалось отправить. Попробуй позже.
support-unavailable = ❌ Поддержка временно недоступна.

    Попробуй позже.
support-cancelled = ❌ Запрос в поддержку отменён.

# === Stale Tasks Inline ===
stale-warning-inline = ⚠️ { $count } { $count ->
    [one] задача не обновлялась
    [few] задачи не обновлялись
    *[other] задач не обновлялись
} 7+ дней

# === Timezone ===
timezone-set-success = ✅ Часовой пояс: { $timezone }

    Напоминания будут приходить по этому времени.

# === Edit Preview ===
edit-preview-voice = 🎤 "{ $text }"

    📝 <b>Превью:</b>

    <s>{ $old_title }</s>
    ↓
    <b>{ $new_title }</b>{ $due_change }

    Применить?

# === Task Display ===
task-display = 📝 { $title }{ $due }{ $reminder }
task-display-due = 📅 { $due }
task-display-reminder = 🔔 Напоминание: { $reminder }
reminder-current = 🔔 Текущее: { $reminder }

# === Edit Preview (text) ===
edit-preview-text = 📝 <b>Превью:</b>

    <s>{ $old_title }</s>
    ↓
    <b>{ $new_title }</b>{ $due_change }

    Применить?

# === Due Date Changes ===
due-change-new = 📅 → { $new }
due-change-update = 📅 { $old } → { $new }
due-change-remove = 📅 { $old } → ❌

# === Task Update Confirmations ===
task-updated-simple = ✅ Обновлено: { $title }
reminder-set-confirm = ⏰ Напоминание установлено!

    📝 { $title }
    🔔 { $reminder }
date-updated-confirm = ✅ Обновлено!

    📝 { $title }{ $due }

# === Admin ===
admin-title = 🔧 <b>Панель администратора</b>
admin-stats = 📊 <b>Статистика</b>
admin-users = 👥 <b>Пользователи</b>
admin-broadcast = 📢 <b>Рассылка</b>
