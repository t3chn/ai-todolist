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
reminder-due = Срок: { $due }

# === Morning Brief ===
brief-title = ☀️ <b>Доброе утро!</b>
brief-tasks-count = У тебя <b>{ $count }</b> { $count ->
    [one] задача
    [few] задачи
    *[other] задач
} на сегодня:
brief-no-tasks = На сегодня задач нет. Хорошего дня! 🌟
brief-tip = 💡 Отправь голосовое сообщение, чтобы быстро добавить задачу!

# === Weekly Review ===
weekly-title = 📊 <b>Итоги недели</b>
weekly-completed = ✅ Выполнено: <b>{ $count }</b> { $count ->
    [one] задача
    [few] задачи
    *[other] задач
}
weekly-created = 📝 Создано: <b>{ $count }</b>
weekly-pending = ⏳ В ожидании: <b>{ $count }</b>
weekly-great-job = 🎉 Отличная работа на этой неделе!
weekly-keep-going = 💪 Так держать!

# === Settings ===
settings-title = ⚙️ <b>Настройки</b>
settings-status = 📊 Статус: { $status }
settings-timezone = 🌍 Часовой пояс: { $tz }
settings-brief-time = ⏰ Утренний дайджест: { $time }
settings-language = 🌐 Язык: { $lang }

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

# === Errors ===
error-generic = ❌ Что-то пошло не так. Попробуй ещё раз.
error-not-found = ❌ Задача не найдена.
error-subscription-required = ⭐ Для этой функции нужна подписка.

# === Admin ===
admin-title = 🔧 <b>Панель администратора</b>
admin-stats = 📊 <b>Статистика</b>
admin-users = 👥 <b>Пользователи</b>
admin-broadcast = 📢 <b>Рассылка</b>
