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

# === Voice Transcription Responses ===
voice-transcribed = 🎤 "{ $text }"
voice-transcribed-added = 🎤 "{ $text }"

    ✅ Added!

    📝 { $title }{ $due }
voice-transcribed-error = 🎤 "{ $text }"

    ❌ Couldn't understand

    💡 Try: "change time to 5pm"
voice-transcribed-reminder-set = 🎤 "{ $text }"

    ⏰ Reminder set!

    📝 { $title }
    🔔 { $reminder }
voice-transcribed-reminder-failed = 🎤 "{ $text }"

    ❌ Couldn't understand the time

    💡 Try: "tomorrow at 9" or "in 2 hours"
voice-transcribed-timezone-set = 🎤 "{ $text }"

    ✅ Timezone set to: { $timezone }
voice-transcribed-timezone-failed = 🎤 "{ $text }"

    ❌ Couldn't determine timezone

    💡 Try a major city name
voice-transcribed-support-sent = 🎤 "{ $text }"

    ✅ Message sent to support!
voice-transcribed-support-failed = ❌ Failed to send message. Please try again.
voice-transcribed-draft = 🎤 "{ $text }"

    ✉️ Draft for { $recipient }:

    { $draft }

    💡 Copy and send!
voice-transcribed-clarify = 🎤 "{ $text }"

    🤔 { $question }

    Pick a specific action:
voice-transcribed-unknown = 🎤 "{ $text }"

    🤖 { $reason }
voice-task-create-failed = ❌ Couldn't create task

    Something went wrong.

    💡 Try typing your task instead

# === Support Flow ===
support-sent-success = ✅ Message sent to support!

    We'll respond within 24 hours.
support-failed = ❌ Failed to send message. Please try again later.
support-unavailable = ❌ Support is temporarily unavailable.

    Please try again later.
support-cancelled = ❌ Support request cancelled.

# === Stale Tasks Inline ===
stale-warning-inline = ⚠️ { $count } { $count ->
    [one] task
    *[other] tasks
} not updated in 7+ days

# === Timezone ===
timezone-set-success = ✅ Timezone set to: { $timezone }

    Your reminders will now use this timezone.

# === Edit Preview ===
edit-preview-voice = 🎤 "{ $text }"

    📝 <b>Preview:</b>

    <s>{ $old_title }</s>
    ↓
    <b>{ $new_title }</b>{ $due_change }

    Apply?

# === Task Display ===
task-display = 📝 { $title }{ $due }{ $reminder }
task-display-due = 📅 { $due }
task-display-reminder = 🔔 Reminder: { $reminder }
reminder-current = 🔔 Current: { $reminder }

# === Edit Preview (text) ===
edit-preview-text = 📝 <b>Preview:</b>

    <s>{ $old_title }</s>
    ↓
    <b>{ $new_title }</b>{ $due_change }

    Apply?

# === Due Date Changes ===
due-change-new = 📅 → { $new }
due-change-update = 📅 { $old } → { $new }
due-change-remove = 📅 { $old } → ❌

# === Task Update Confirmations ===
task-updated-simple = ✅ Updated: { $title }
reminder-set-confirm = ⏰ Reminder set!

    📝 { $title }
    🔔 { $reminder }
date-updated-confirm = ✅ Updated!

    📝 { $title }{ $due }

# === Admin Panel ===
admin-title = 🔐 <b>Admin Panel</b>

    Select an option:
admin-access-denied = ⛔ Access denied

# Admin Buttons
btn-admin-stats = 📊 Stats
btn-admin-users = 👥 Users
btn-admin-broadcast = 📢 Broadcast
btn-admin-banned = 🚫 Banned
btn-admin-search = 🔍 Search
btn-admin-recent = 📋 Recent
btn-admin-back = ↩️ Back
btn-admin-cancel = ↩️ Cancel
btn-admin-all-users = 📢 All users
btn-admin-trial = 🎁 Trial
btn-admin-paid = ✅ Paid
btn-admin-expired = ❌ Expired
btn-admin-grant-30 = 📅 +30 days
btn-admin-grant-90 = 📅 +90 days
btn-admin-grant-365 = 📅 +365 days
btn-admin-unban = ✅ Unban
btn-admin-ban = 🚫 Ban
btn-admin-message = 💬 Message

# Admin Stats
admin-stats-title = 📊 <b>Statistics</b>

    👥 <b>Users</b>
    ├─ Total: <b>{ $total }</b>
    ├─ 🎁 Trial: <b>{ $trial }</b>
    ├─ ✅ Paid: <b>{ $paid }</b>
    ├─ ❌ Expired: <b>{ $expired }</b>
    └─ 🚫 Banned: <b>{ $banned }</b>

    📈 <b>Activity</b>
    ├─ Active 7d: <b>{ $active7d }</b>
    └─ Active 30d: <b>{ $active30d }</b>

    🆕 <b>Growth</b>
    ├─ Today: <b>+{ $today }</b>
    └─ This week: <b>+{ $week }</b>

# Admin User Management
admin-users-title = 👥 <b>User Management</b>

    Search by @username, name or ID:
admin-search-title = 🔍 <b>Search User</b>

    Send @username, name or telegram ID:
admin-search-prompt = Enter search query
admin-search-results = 🔍 Found { $count } user(s):
admin-recent-title = 📋 <b>Recent Users</b>

# Admin Broadcast
admin-broadcast-title = 📢 <b>Broadcast</b>

    Select target audience:
admin-broadcast-to = 📢 <b>Broadcast to { $segment }</b>

    Send your message:
admin-broadcast-prompt = Send broadcast message
admin-broadcast-sending = 📢 Sending to { $count } users...
admin-broadcast-complete = ✅ Broadcast complete!

    📨 Sent: { $sent }
    ❌ Failed: { $failed }
admin-segment-all = all users
admin-segment-trial = trial users
admin-segment-paid = paid users
admin-segment-expired = expired users

# Admin Banned Users
admin-banned-title = 🚫 <b>Banned Users</b>
admin-banned-empty = 🚫 <b>Banned Users</b>

    No banned users.
admin-ban-no-reason = No reason

# Admin User Card
admin-user-card = 👤 <b>{ $name }</b>
    ID: <code>{ $id }</code> | TG: <code>{ $tg_id }</code>

    📊 Status: { $status }
    📅 Joined: { $joined }
    📝 Tasks: { $tasks }
    🔗 Referrals: { $referrals }

# Admin Actions
admin-grant-success = ✅ Granted { $days } days
admin-grant-confirm = ✅ Subscription granted!

    👤 { $name } — { $status }
admin-grant-failed = ❌ Failed
admin-ban-reason = Banned by admin
admin-ban-callback = 🚫 User banned
admin-ban-confirm = 🚫 User has been banned.
admin-unban-callback = ✅ User unbanned
admin-unban-confirm = ✅ User has been unbanned.

# Admin Message
admin-message-title = 💬 <b>Send Message</b>

    Type your message to send to this user:
admin-message-prompt = Type message
admin-message-sent = ✅ Message sent to { $name }
admin-message-failed = ❌ Failed to send: { $error }
admin-message-from = 📨 <b>Message from Admin:</b>

    { $text }
admin-text-required = ⚠️ Please send text message for admin actions.
