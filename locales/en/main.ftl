# English translations for AI Todolist Bot

# === Welcome & Start ===
welcome = <b>{ $name }</b>, welcome!

    I'm your AI task assistant.
    Text or voice — I'll understand.

    <code>Call mom tomorrow at 5pm</code>
    Try it now ↑

welcome-trial = 🎁 You have a <b>7-day free trial</b>!
welcome-referral-bonus = 🎉 +7 days bonus from referral!

# === Help ===
help-text = <b>Available commands:</b>

    /tasks — List your tasks
    /today — Today's tasks
    /settings — Settings & subscription
    /invite — Invite friends, get +7 days
    /support — Contact support

    Just send me a message to create a task!

# === Tasks ===
task-created = ✅ Task created: <b>{ $title }</b>
task-created-with-reminder = ✅ Task created: <b>{ $title }</b>
    ⏰ Reminder: { $reminder }
task-done = ✅ Done: <b>{ $title }</b>
task-deleted = 🗑 Deleted: { $title }
task-updated = ✏️ Updated: <b>{ $title }</b>

tasks-empty = 📋 No tasks yet!

    Send me a message to create one.
tasks-header = 📋 Your tasks ({ $count }):
tasks-today-empty = 📅 No tasks for today.

    Enjoy your free time! 🌴
tasks-today-header = 📅 Today's tasks ({ $count }):

task-status-pending = ⏳
task-status-in-progress = 🔄
task-status-done = ✅

# === Task Actions ===
task-snooze-1h = Snoozed for 1 hour
task-snooze-tomorrow = Snoozed until tomorrow
task-reminder-snoozed = ⏰ Reminder snoozed: { $title }

# === Reminders ===
reminder-title = ⏰ <b>Reminder</b>
reminder-task = { $title }
reminder-due = 📅 Due: { $due }
reminder-urgent = 🔴 Due very soon!
reminder-soon = 🟡 Due in 30 minutes
reminder-normal = ⏰ Reminder

# === Morning Brief ===
brief-title = ☀️ <b>Good morning!</b>
brief-greeting-morning = 👋 Good morning, { $name }!
brief-greeting-afternoon = 👋 Good afternoon, { $name }!
brief-greeting-evening = 👋 Good evening, { $name }!
brief-tasks-count = You have <b>{ $count }</b> { $count ->
    [one] task
    *[other] tasks
} for today:
brief-today-header = 📅 Today's tasks ({ $count }):
brief-no-tasks = 📅 No tasks scheduled for today.
brief-other-pending = 📋 { $count } other pending { $count ->
    [one] task
    *[other] tasks
}
brief-outro = Have a productive day! 🚀
brief-tip = 💡 Send me a voice message to quickly add tasks!
btn-view-tasks = 📋 View tasks

# === Weekly Review ===
weekly-title = 📊 <b>Weekly Review</b>
weekly-greeting = Hi { $name }!
weekly-this-week = This week:
weekly-completed = ✅ Completed: <b>{ $count }</b> { $count ->
    [one] task
    *[other] tasks
}
weekly-created = ➕ Added: <b>{ $count }</b>
weekly-pending = 📋 Pending: <b>{ $count }</b>
weekly-great-job = 🎉 Great job this week!
weekly-keep-going = 💪 Keep going!
weekly-celebrate-good = Good progress! 👍
weekly-celebrate-great = Great week! 🔥
weekly-celebrate-productive = Productive week! ⚡
weekly-celebrate-incredible = Incredible productivity! 🏆
weekly-stale-warning = ⚠️ { $count } { $count ->
    [one] task is
    *[other] tasks are
} stale (7+ days). Time to review?
weekly-outro = Have a great week ahead! 🚀
btn-review-stale = 📋 Review stale
btn-view-all = 📊 View all

# === Settings ===
settings-title = ⚙️ <b>Settings</b>
settings-status = 📊 Status: { $status }
settings-timezone = 🌍 Timezone: { $tz }
settings-brief-time = ⏰ Morning brief: { $time }
settings-language = 🌐 Language: { $lang }
settings-full = ⚙️ <b>Settings</b>

    📊 Status: { $status }
    🌍 Timezone: { $tz }
    ⏰ Morning brief: { $time }
    🌐 Language: { $lang }

settings-timezone-title = 🌍 <b>Select timezone:</b>
settings-brief-title = ⏰ <b>Morning brief time:</b>
settings-language-title = 🌐 <b>Select language:</b>

settings-updated = ✅ Settings updated!
settings-timezone-updated = ✅ Timezone set to { $tz }
settings-brief-updated = ✅ Morning brief set to { $time }
settings-language-updated = ✅ Language changed to { $lang }

# === Subscription ===
subscription-trial = 🎁 Trial ({ $days } { $days ->
    [one] day
    *[other] days
})
subscription-active = ✅ Active ({ $days } { $days ->
    [one] day
    *[other] days
})
subscription-trial-warning = ⚠️ Trial ends in { $days } { $days ->
    [one] day
    *[other] days
}
subscription-expired = ❌ Expired

subscription-expired-message = ⏰ Your trial has ended!

    To continue using AI Todolist, subscribe for just <b>⭐50 Stars/month</b>.

    ✅ Unlimited tasks
    ✅ Voice messages
    ✅ Smart reminders
    ✅ Morning briefs

subscription-success = 🎉 Thank you for subscribing!

    ✅ Your { $type } subscription is now active until { $expires }.

    Enjoy AI Todolist! 🚀

# === Referrals ===
invite-title = 🎁 <b>Invite Friends</b>
invite-description = Share your link and get <b>+7 days</b> for each friend who joins!
invite-link = Your referral link:
invite-stats = 👥 Invited: <b>{ $count }</b>
    🎁 Bonus days: <b>{ $bonus }</b>
invite-share = 📲 Share:
invite-full = 🎁 <b>Invite Friends</b>

    Share your link:
    <code>https://t.me/{ $bot }?start={ $code }</code>

    ✨ You both get +7 days free!

    👥 Invited: { $count } · Bonus: { $bonus } days

# === Support ===
support-prompt = 📮 <b>Contact Support</b>

    Send your message below and we'll get back to you soon.
support-sent = ✅ Message sent to support. We'll reply soon!

# === Stale Tasks ===
stale-warning = ⚠️ { $count } { $count ->
    [one] task hasn't
    *[other] tasks haven't
} been updated in 7+ days.

    Consider reviewing or completing them!

# === Duplicate Detection ===
duplicate-warning = ⚠️ Similar task exists:
    <i>{ $title }</i>

    Create anyway?

# === Celebration ===
celebration-streak = 🔥 <b>{ $count }-task streak!</b> Keep it up!
celebration-milestone = 🏆 <b>Milestone!</b> You've completed { $count } tasks!
celebrate-first = 🎯 First task done!
celebrate-keep-going = 👍 Keep going!
celebrate-on-fire = 🔥 On fire!
celebrate-unstoppable = 💪 Unstoppable!

# === Task Completion ===
task-completed-stats = ✅ <b>{ $title }</b>

    { $celebration }
    📊 { $done }/{ $total } done today
task-next = 📝 Next: <b>{ $title }</b>{ $due }
task-all-done = 🎉 All done! You completed { $count } { $count ->
    [one] task
    *[other] tasks
} today!
task-delete-confirm = 🗑 Delete <b>{ $title }</b>?
task-snoozed = ⏰ Snoozed: <b>{ $title }</b>

    I'll remind you { $when }.
task-deleted-msg = 🗑 Deleted: { $title }
task-cancelled = Cancelled
task-added = ✅ Added!

    📝 { $title }{ $due }
task-create-failed = ❌ Couldn't create task
session-expired = Session expired

# === Stale Tasks ===
stale-no-tasks = ✅ No stale tasks!
stale-reviewing = 📋 Reviewing { $count } stale { $count ->
    [one] task
    *[other] tasks
}...
stale-task-item = 🕐 { $title }

    📅 Last updated: { $updated }
stale-kept-all = ✅ Kept { $count } { $count ->
    [one] task
    *[other] tasks
}. They won't appear as stale for 7 more days.
stale-kept-one = ✅ Kept: { $title }
btn-keep = ✅ Keep
btn-review = 📋 Review
btn-keep-all = ✓ Keep all

# === Timezone ===
tz-select-title = 🌍 Select your timezone:

    📍 Auto-detect uses your location
    🏙 Type city lets you enter any city
tz-auto-prompt = 📍 Tap the button below to share your location.

    I'll detect your timezone automatically.
tz-city-prompt = 🏙 Type your city name:

    Examples: Moscow, New York, Tokyo, Dubai
tz-updated = ✅ Timezone set to: { $tz }
btn-auto-detect = 📍 Auto-detect
btn-type-city = 🏙 Type city

# === Brief Time ===
brief-select-title = ⏰ Select morning brief time:
brief-updated = ✅ Morning brief time set to: { $time }

# === Language ===
lang-select-title = 🌐 Select language / Выберите язык:
lang-updated-en = ✅ Language changed to English
lang-updated-ru = ✅ Язык изменён на Русский

# === Voice ===
voice-processing = 🎤 Processing voice message...
voice-transcribed = 📝 Transcribed: <i>{ $text }</i>

# === Draft ===
draft-created = 📝 <b>Message draft:</b>

    { $text }

    <i>Copy and send when ready!</i>

# === Vague Task ===
vague-suggestion = 💡 This task seems broad. Want to break it down?

    Suggestions:
    { $suggestions }

# === Tags ===
tag-work = 💼 Work
tag-personal = 🏠 Personal
tag-shopping = 🛒 Shopping
tag-health = 🏥 Health
tag-finance = 💰 Finance
tag-other = 📌 Other

# === Buttons ===
btn-settings = ⚙️ Settings
btn-done = ✅ Done
btn-delete = 🗑 Delete
btn-edit = ✏️ Edit
btn-snooze-1h = ⏰ 1h
btn-snooze-tomorrow = 📅 Tomorrow
btn-back = ← Back
btn-timezone = 🌍 Timezone
btn-brief-time = ⏰ Brief time
btn-language = 🌐 Language
btn-invite = 🎁 Invite friends
btn-subscribe = ⭐ Subscribe
btn-create-anyway = ✅ Create anyway
btn-cancel = ❌ Cancel
btn-yes = ✅ Yes
btn-no = ❌ No
btn-do-it = ✅ Do it
btn-all-tasks = 📋 All tasks
btn-yes-delete = 🗑 Yes, delete
snooze-1h = 1 hour
snooze-tomorrow = tomorrow
snooze-later = later

# === Errors ===
error-generic = ❌ Something went wrong. Please try again.
error-not-found = ❌ Task not found.
error-subscription-required = ⭐ Subscription required to use this feature.
error-start-first = Please /start first
error-use-buttons = ⚠️ Please use the buttons above to confirm or cancel.
error-task-create = ❌ Couldn't create task

    Something went wrong on our end.

    💡 Try: "Buy milk tomorrow at 5pm"
error-voice-failed = 🎤 Couldn't understand voice

    The audio wasn't clear enough.

    💡 Try speaking closer to mic or type your task
error-voice-requires-ai = ❌ Voice messages require AI service
error-ai-required = ❌ AI service required
error-edit-failed = ❌ Couldn't understand the edit instruction

    💡 Try: "change time to 5pm" or "replace John with Mike"
error-timezone-failed = ❌ Couldn't determine timezone

    💡 Try a major city name like "Moscow" or "New York"
error-reminder-time-failed = ❌ Couldn't understand the time

    💡 Try: "tomorrow at 9" or "in 2 hours"

# === Voice ===
voice-processing = 🎤 Processing voice...

# === Edit ===
edit-title = ✏️ Edit task:

    📝 { $title }{ $due }

    What would you like to change?
edit-send-title = 📝 Current: { $title }

    Send new title:
edit-preview = ✏️ Preview changes:

    📝 { $old_title } → { $new_title }{ $due_change }

    Apply these changes?
edit-cancelled = ❌ Edit cancelled
edit-applied = ✅ Updated!

    📝 { $title }{ $due }
btn-apply = ✅ Apply
btn-edit-title = 📝 Edit title
btn-edit-date = 📅 Edit date

# === Date Selection ===
date-select-title = 📅 Select new due date:
btn-date-today = 📅 Today
btn-date-tomorrow = 📅 Tomorrow
btn-date-next-week = 📅 Next week
btn-date-remove = 🚫 Remove date
date-updated = ✅ Date updated

# === Reminder Selection ===
remind-select-title = ⏰ Set reminder for:

    📝 { $title }

    { $current }

    ✍️ Custom: send text or 🎤 voice
remind-custom-prompt = ⏰ When to remind about:
    📝 { $title }

    Send time (text or 🎤 voice):
    • "tomorrow at 3pm"
    • "in 2 hours"
    • "monday morning"
btn-remind-30min = ⏰ 30 min
btn-remind-1h = ⏰ 1 hour
btn-remind-3h = ⏰ 3 hours
btn-remind-tomorrow = ⏰ Tomorrow 9am
btn-remind-custom = ✍️ Custom
btn-remind-remove = 🚫 Remove
remind-removed = 🔕 Reminder removed
remind-set = ⏰ Reminder set { $when }
no-reminder = No reminder set

# === Stale Tasks ===
btn-stale-review = 📋 Review
btn-stale-keep-all = ✓ Keep all

# === Subscription Expired ===
subscription-expired-full = ⏰ Your trial has ended!

    To continue using AI Todolist, subscribe:

    ⭐ 1 month — 250 Stars (~$5)
    ⭐ 3 months — 600 Stars (~$12)
    ⭐ 12 months — 2000 Stars (~$40)

    Use /settings → Subscribe

# === Draft ===
draft-message = ✉️ Draft for { $recipient }:

    { $text }

    💡 Copy and send!

# === Clarify ===
clarify-prompt = 🤔 { $question }

    Pick a specific action:
btn-create-as-is = 📝 Create as-is

# === AI Commands ===
ai-tasks-empty = 📋 No pending tasks!

    💡 Send a task like: "Call mom tomorrow"
ai-tasks-more = ...and { $count } more
ai-today-empty = 📅 No tasks for today!

    💡 Add one: "Meeting at 3pm"
ai-today-header = 📅 Today ({ $count } { $count ->
    [one] task
    *[other] tasks
}):
ai-unknown-command = 🤖 Unknown command

# === Admin ===
admin-title = 🔧 <b>Admin Panel</b>
admin-stats = 📊 <b>Statistics</b>
admin-users = 👥 <b>Users</b>
admin-broadcast = 📢 <b>Broadcast</b>
