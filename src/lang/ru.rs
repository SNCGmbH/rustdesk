lazy_static::lazy_static! {
pub static ref T: std::collections::HashMap<&'static str, &'static str> =
    [
        ("Status", "Статус"),
        ("Your Desktop", "Ваш рабочий стол"),
        ("desk_tip", "Ваш рабочий стол доступен с этим ID и паролем"),
        ("Password", "Пароль"),
        ("Ready", "Готово"),
        ("Established", "Установлено"),
        ("connecting_status", "Подключение к сети RustDesk..."),
        ("Enable Service", "Включить службу"),
        ("Start Service", "Запустить службу"),
        ("Service is running", "Служба запущена"),
        ("Service is not running", "Служба не запущена"),
        ("not_ready_status", "Не выполнено. Проверьте подключение."),
        ("Control Remote Desktop", "Управление удалённым рабочим столом"),
        ("Transfer File", "Передать файл"),
        ("Connect", "Подключиться"),
        ("Recent Sessions", "Последние сеансы"),
        ("Address Book", "Адресная книга"),
        ("Confirmation", "Подтверждение"),
        ("TCP Tunneling", "TCP-туннелирование"),
        ("Remove", "Удалить"),
        ("Refresh random password", "Обновить случайный пароль"),
        ("Set your own password", "Установить свой пароль"),
        ("Enable Keyboard/Mouse", "Включить клавиатуру/мышь"),
        ("Enable Clipboard", "Включить буфер обмена"),
        ("Enable File Transfer", "Включить передачу файлов"),
        ("Enable TCP Tunneling", "Включить туннелирование TCP"),
        ("IP Whitelisting", "Список разрешённых IP-адресов"),
        ("ID/Relay Server", "ID/Сервер ретрансляции"),
        ("Import Server Config", "Импортировать конфигурацию сервера"),
        ("Export Server Config", "Экспортировать конфигурацию сервера"),
        ("Import server configuration successfully", "Конфигурация сервера успешно импортирована"),
        ("Export server configuration successfully", "Конфигурация сервера успешно экспортирована"),
        ("Invalid server configuration", "Неправильная конфигурация сервера"),
        ("Clipboard is empty", "Буфер обмена пуст"),
        ("Stop service", "Остановить службу"),
        ("Change ID", "Изменить ID"),
        ("Website", "Сайт"),
        ("About", "О программе"),
        ("Slogan_tip", "Сделано с душой в этом безумном мире!"),
        ("Privacy Statement", "Заявление о конфиденциальности"),
        ("Mute", "Отключить звук"),
        ("Build Date", "Дата сборки"),
        ("Version", "Версия"),
        ("Home", "Главная"),
        ("Audio Input", "Аудиовход"),
        ("Enhancements", "Улучшения"),
        ("Hardware Codec", "Аппаратный кодек"),
        ("Adaptive Bitrate", "Адаптивная скорость потока"),
        ("ID Server", "ID-сервер"),
        ("Relay Server", "Сервер ретрансляции"),
        ("API Server", "API-сервер"),
        ("invalid_http", "Должен начинаться с http:// или https://"),
        ("Invalid IP", "Неправильный IP-адрес"),
        ("id_change_tip", "Допускаются только символы a-z, A-Z, 0-9 и _ (подчёркивание). Первой должна быть буква a-z, A-Z. Длина от 6 до 16."),
        ("Invalid format", "Неправильный формат"),
        ("server_not_support", "Пока не поддерживается сервером"),
        ("Not available", "Недоступно"),
        ("Too frequent", "Слишком часто"),
        ("Cancel", "Отменить"),
        ("Skip", "Пропустить"),
        ("Close", "Закрыть"),
        ("Retry", "Повторить"),
        ("OK", "ОК"),
        ("Password Required", "Требуется пароль"),
        ("Please enter your password", "Введите пароль"),
        ("Remember password", "Запомнить пароль"),
        ("Wrong Password", "Неправильный пароль"),
        ("Do you want to enter again?", "Повторить вход?"),
        ("Connection Error", "Ошибка подключения"),
        ("Error", "Ошибка"),
        ("Reset by the peer", "Сброшено удалённым узлом"),
        ("Connecting...", "Подключение..."),
        ("Connection in progress. Please wait.", "Выполняется подключение. Подождите."),
        ("Please try 1 minute later", "Попробуйте через минуту"),
        ("Login Error", "Ошибка входа"),
        ("Successful", "Успешно"),
        ("Connected, waiting for image...", "Подключено, ожидание изображения..."),
        ("Name", "Имя"),
        ("Type", "Тип"),
        ("Modified", "Изменено"),
        ("Size", "Размер"),
        ("Show Hidden Files", "Показать скрытые файлы"),
        ("Receive", "Получить"),
        ("Send", "Отправить"),
        ("Refresh File", "Обновить файл"),
        ("Local", "Локальный"),
        ("Remote", "Удалённый"),
        ("Remote Computer", "Удалённый компьютер"),
        ("Local Computer", "Локальный компьютер"),
        ("Confirm Delete", "Подтвердить удаление"),
        ("Delete", "Удалить"),
        ("Properties", "Свойства"),
        ("Multi Select", "Многоэлементный выбор"),
        ("Select All", "Выбрать все"),
        ("Unselect All", "Снять все"),
        ("Empty Directory", "Пустая папка"),
        ("Not an empty directory", "Папка не пуста"),
        ("Are you sure you want to delete this file?", "Удалить этот файл?"),
        ("Are you sure you want to delete this empty directory?", "Удалить пустую папку?"),
        ("Are you sure you want to delete the file of this directory?", "Удалить файл из этой папки?"),
        ("Do this for all conflicts", "Применить ко всем конфликтам"),
        ("This is irreversible!", "Это необратимо!"),
        ("Deleting", "Удаление"),
        ("files", "файлы"),
        ("Waiting", "Ожидание"),
        ("Finished", "Завершено"),
        ("Speed", "Скорость"),
        ("Custom Image Quality", "Пользовательское качество изображения"),
        ("Privacy mode", "Режим конфиденциальности"),
        ("Block user input", "Блокировать пользовательский ввод"),
        ("Unblock user input", "Разблокировать пользовательский ввод"),
        ("Adjust Window", "Настроить окно"),
        ("Original", "Оригинал"),
        ("Shrink", "Уменьшить"),
        ("Stretch", "Растянуть"),
        ("Scrollbar", "Полоса прокрутки"),
        ("ScrollAuto", "Автопрокрутка"),
        ("Good image quality", "Хорошее качество изображения"),
        ("Balanced", "Сбалансировано"),
        ("Optimize reaction time", "Оптимальное время реакции"),
        ("Custom", "Своё"),
        ("Show remote cursor", "Показать удалённый курсор"),
        ("Show quality monitor", "Показать качество"),
        ("Disable clipboard", "Отключить буфер обмена"),
        ("Lock after session end", "Выход из учётной записи после завершения сеанса"),
        ("Insert", "Вставить"),
        ("Insert Lock", "Установить замок"),
        ("Refresh", "Обновить"),
        ("ID does not exist", "ID не существует"),
        ("Failed to connect to rendezvous server", "Невозможно подключиться к промежуточному серверу"),
        ("Please try later", "Пожалуйста, попробуйте позже"),
        ("Remote desktop is offline", "Удалённый рабочий стол не в сети"),
        ("Key mismatch", "Несоответствие ключей"),
        ("Timeout", "Истекло время ожидания"),
        ("Failed to connect to relay server", "Невозможно подключиться к серверу ретрансляции"),
        ("Failed to connect via rendezvous server", "Невозможно подключиться через промежуточный сервер"),
        ("Failed to connect via relay server", "Невозможно подключиться через сервер ретрансляции"),
        ("Failed to make direct connection to remote desktop", "Невозможно установить прямое подключение к удалённому рабочему столу"),
        ("Set Password", "Установить пароль"),
        ("OS Password", "Пароль ОС"),
        ("install_tip", "В некоторых случаях из-за UAC RustDesk может работать неправильно на удалённом узле. Чтобы избежать UAC, нажмите кнопку ниже, чтобы установить RustDesk в системе."),
        ("Click to upgrade", "Нажмите для проверки обновлений"),
        ("Click to download", "Нажмите для скачивания"),
        ("Click to update", "Нажмите для обновления"),
        ("Configure", "Настроить"),
        ("config_acc", "Чтобы удалённо управлять своим рабочим столом, вы должны предоставить RustDesk права \"доступа\""),
        ("config_screen", "Для удалённого доступа к рабочему столу вы должны предоставить RustDesk права \"снимок экрана\""),
        ("Installing ...", "Установка..."),
        ("Install", "Установить"),
        ("Installation", "Установка"),
        ("Installation Path", "Путь установки"),
        ("Create start menu shortcuts", "Создать ярлыки меню \"Пуск\""),
        ("Create desktop icon", "Создать значок на рабочем столе"),
        ("agreement_tip", "Начиная установку, вы принимаете условия лицензионного соглашения"),
        ("Accept and Install", "Принять и установить"),
        ("End-user license agreement", "Лицензионное соглашение с конечным пользователем"),
        ("Generating ...", "Генерация..."),
        ("Your installation is lower version.", "Установлена более ранняя версия"),
        ("not_close_tcp_tip", "Не закрывать это окно при использовании туннеля"),
        ("Listening ...", "Ожидание..."),
        ("Remote Host", "Удалённый узел"),
        ("Remote Port", "Удалённый порт"),
        ("Action", "Действие"),
        ("Add", "Добавить"),
        ("Local Port", "Локальный порт"),
        ("Local Address", "Локальный адрес"),
        ("Change Local Port", "Изменить локальный порт"),
        ("setup_server_tip", "Для более быстрого подключения настройте свой собственный сервер подключения"),
        ("Too short, at least 6 characters.", "Слишком коротко, минимум 6 символов"),
        ("The confirmation is not identical.", "Подтверждение не совпадает"),
        ("Permissions", "Разрешения"),
        ("Accept", "Принять"),
        ("Dismiss", "Отклонить"),
        ("Disconnect", "Отключить"),
        ("Allow using keyboard and mouse", "Разрешить использование клавиатуры и мыши"),
        ("Allow using clipboard", "Разрешить использование буфера обмена"),
        ("Allow hearing sound", "Разрешить передачу звука"),
        ("Allow file copy and paste", "Разрешить копирование и вставку файлов"),
        ("Connected", "Подключено"),
        ("Direct and encrypted connection", "Прямое и зашифрованное соединение"),
        ("Relayed and encrypted connection", "Ретранслируемое и зашифрованное соединение"),
        ("Direct and unencrypted connection", "Прямое и незашифрованное соединение"),
        ("Relayed and unencrypted connection", "Ретранслируемое и незашифрованное соединение"),
        ("Enter Remote ID", "Введите удалённый ID"),
        ("Enter your password", "Введите пароль"),
        ("Logging in...", "Вход..."),
        ("Enable RDP session sharing", "Включить общий доступ к сеансу RDP"),
        ("Auto Login", "Автоматический вход (действителен только если вы установили \"Завершение пользовательского сеанса после завершения удалённого подключения\""),
        ("Enable Direct IP Access", "Включить прямой IP-доступ"),
        ("Rename", "Переименовать"),
        ("Space", "Место"),
        ("Create Desktop Shortcut", "Создать ярлык на рабочем столе"),
        ("Change Path", "Изменить путь"),
        ("Create Folder", "Создать папку"),
        ("Please enter the folder name", "Введите имя папки"),
        ("Fix it", "Исправить"),
        ("Warning", "Предупреждение"),
        ("Login screen using Wayland is not supported", "Вход в систему с использованием Wayland не поддерживается"),
        ("Reboot required", "Требуется перезагрузка"),
        ("Unsupported display server ", "Неподдерживаемый сервер отображения"),
        ("x11 expected", "Ожидается X11"),
        ("Port", "Порт"),
        ("Settings", "Настройки"),
        ("Username", "Пользователь"),
        ("Invalid port", "Неправильный порт"),
        ("Closed manually by the peer", "Закрыто удалённым узлом вручную"),
        ("Enable remote configuration modification", "Разрешить удалённое изменение конфигурации"),
        ("Run without install", "Запустить без установки"),
        ("Always connected via relay", "Всегда подключается через ретрансляционный сервер"),
        ("Always connect via relay", "Всегда подключаться через ретрансляционный сервер"),
        ("whitelist_tip", "Только IP-адреса из белого списка могут получить доступ ко мне"),
        ("Login", "Войти"),
        ("Verify", "Проверить"),
        ("Remember me", "Запомнить"),
        ("Trust this device", "Доверенное устройство"),
        ("Verification code", "Проверочный код"),
        ("verification_tip", "Обнаружено новое устройство, на зарегистрированный адрес электронной почты отправлен проверочный код. Введите его, чтобы продолжить вход в систему."),
        ("Logout", "Выйти"),
        ("Tags", "Метки"),
        ("Search ID", "Поиск по ID"),
        ("whitelist_sep", "Раздельно запятой, точкой с запятой, пробелом или новой строкой"),
        ("Add ID", "Добавить ID"),
        ("Add Tag", "Добавить ключевое слово"),
        ("Unselect all tags", "Отменить выбор всех меток"),
        ("Network error", "Ошибка сети"),
        ("Username missed", "Имя пользователя отсутствует"),
        ("Password missed", "Забыли пароль"),
        ("Wrong credentials", "Неправильные учётные данные"),
        ("Edit Tag", "Изменить метку"),
        ("Unremember Password", "Не сохранять пароль"),
        ("Favorites", "Избранное"),
        ("Add to Favorites", "Добавить в избранное"),
        ("Remove from Favorites", "Удалить из избранного"),
        ("Empty", "Пусто"),
        ("Invalid folder name", "Недопустимое имя папки"),
        ("Socks5 Proxy", "SOCKS5-прокси"),
        ("Hostname", "Узел"),
        ("Discovered", "Найдено"),
        ("install_daemon_tip", "Для запуска при загрузке необходимо установить системную службу"),
        ("Remote ID", "Удалённый ID"),
        ("Paste", "Вставить"),
        ("Paste here?", "Вставить сюда?"),
        ("Are you sure to close the connection?", "Завершить подключение?"),
        ("Download new version", "Скачать новую версию"),
        ("Touch mode", "Сенсорный режим"),
        ("Mouse mode", "Режим мыши"),
        ("One-Finger Tap", "Касание одним пальцем"),
        ("Left Mouse", "Левая кнопка мыши"),
        ("One-Long Tap", "Долгое касание одним пальцем"),
        ("Two-Finger Tap", "Касание двумя пальцами"),
        ("Right Mouse", "Правая кнопка мыши"),
        ("One-Finger Move", "Перемещение одним пальцем"),
        ("Double Tap & Move", "Двойное касание и перемещение"),
        ("Mouse Drag", "Перетаскивание мышью"),
        ("Three-Finger vertically", "Тремя пальцами по вертикали"),
        ("Mouse Wheel", "Колёсико мыши"),
        ("Two-Finger Move", "Перемещение двумя пальцами"),
        ("Canvas Move", "Перемещение холста"),
        ("Pinch to Zoom", "Масштабирование щипком"),
        ("Canvas Zoom", "Масштаб холста"),
        ("Reset canvas", "Сбросить холст"),
        ("No permission of file transfer", "Нет разрешения на передачу файлов"),
        ("Note", "Примечание"),
        ("Connection", "Соединение"),
        ("Share Screen", "Поделиться экраном"),
        ("CLOSE", "ЗАКРЫТЬ"),
        ("OPEN", "ОТКРЫТЬ"),
        ("Chat", "Чат"),
        ("Total", "Всего"),
        ("items", "элементы"),
        ("Selected", "Выбрано"),
        ("Screen Capture", "Захват экрана"),
        ("Input Control", "Входной контроль"),
        ("Audio Capture", "Захват аудио"),
        ("File Connection", "Файловое соединение"),
        ("Screen Connection", "Подключение экрана"),
        ("Do you accept?", "Вы согласны?"),
        ("Open System Setting", "Открыть настройки системы"),
        ("How to get Android input permission?", "Как получить разрешение на ввод Android?"),
        ("android_input_permission_tip1", "Чтобы удалённое устройство могло управлять вашим Android-устройством с помощью мыши или касания, вам необходимо разрешить RustDesk использовать службу \"Специальные возможности\"."),
        ("android_input_permission_tip2", "Перейдите на следующую страницу системных настроек, найдите и войдите в \"Установленные службы\", включите службу \"RustDesk Input\"."),
        ("android_new_connection_tip", "Получен новый запрос на управление вашим текущим устройством."),
        ("android_service_will_start_tip", "Включение захвата экрана автоматически запускает службу, позволяя другим устройствам запрашивать соединение с этим устройством."),
        ("android_stop_service_tip", "Закрытие службы автоматически закроет все установленные соединения."),
        ("android_version_audio_tip", "Текущая версия Android не поддерживает захват звука, обновите её до Android 10 или выше."),
        ("android_start_service_tip", "Нажмите \"Запуск промежуточного сервера\" или ОТКРЫТЬ разрешение \"Захват экрана\", чтобы запустить службу демонстрации экрана."),
        ("Account", "Аккаунт"),
        ("Overwrite", "Перезаписать"),
        ("This file exists, skip or overwrite this file?", "Этот файл существует, пропустить или перезаписать файл?"),
        ("Quit", "Выйти"),
        ("doc_mac_permission", "https://rustdesk.com/docs/ru/manual/mac/#включение-разрешений"),
        ("Help", "Помощь"),
        ("Failed", "Не выполнено"),
        ("Succeeded", "Выполнено"),
        ("Someone turns on privacy mode, exit", "Кто-то включает режим конфиденциальности, выход"),
        ("Unsupported", "Не поддерживается"),
        ("Peer denied", "Отклонено удалённым узлом"),
        ("Please install plugins", "Установите плагины"),
        ("Peer exit", "Удалённый узел отключён"),
        ("Failed to turn off", "Невозможно отключить"),
        ("Turned off", "Отключён"),
        ("In privacy mode", "В режиме конфиденциальности"),
        ("Out privacy mode", "Выход из режима конфиденциальности"),
        ("Language", "Язык"),
        ("Keep RustDesk background service", "Держать в фоне службу RustDesk"),
        ("Ignore Battery Optimizations", "Игнорировать оптимизацию батареи"),
        ("android_open_battery_optimizations_tip", "Перейдите на следующую страницу настроек"),
        ("Connection not allowed", "Подключение не разрешено"),
        ("Legacy mode", "Устаревший режим"),
        ("Map mode", "Режим сопоставления"),
        ("Translate mode", "Режим перевода"),
        ("Use permanent password", "Использовать постоянный пароль"),
        ("Use both passwords", "Использовать оба пароля"),
        ("Set permanent password", "Установить постоянный пароль"),
        ("Enable Remote Restart", "Включить удалённый перезапуск"),
        ("Allow remote restart", "Разрешить удалённый перезапуск"),
        ("Restart Remote Device", "Перезапустить удалённое устройство"),
        ("Are you sure you want to restart", "Вы уверены, что хотите выполнить перезапуск?"),
        ("Restarting Remote Device", "Перезагрузка удалённого устройства"),
        ("remote_restarting_tip", "Удалённое устройство перезапускается. Закройте это сообщение и через некоторое время переподключитесь, используя постоянный пароль."),
        ("Copied", "Скопировано"),
        ("Exit Fullscreen", "Выйти из полноэкранного режима"),
        ("Fullscreen", "Полноэкранный"),
        ("Mobile Actions", "Мобильные действия"),
        ("Select Monitor", "Выберите монитор"),
        ("Control Actions", "Действия по управлению"),
        ("Display Settings", "Настройки отображения"),
        ("Ratio", "Соотношение"),
        ("Image Quality", "Качество изображения"),
        ("Scroll Style", "Стиль прокрутки"),
        ("Show Menubar", "Показать строку меню"),
        ("Hide Menubar", "Скрыть строку меню"),
        ("Direct Connection", "Прямая связь"),
        ("Relay Connection", "Ретранслируемое соединение"),
        ("Secure Connection", "Безопасное соединение"),
        ("Insecure Connection", "Небезопасное соединение"),
        ("Scale original", "Оригинальный масштаб"),
        ("Scale adaptive", "Адаптивный масштаб"),
        ("General", "Общие"),
        ("Security", "Безопасность"),
        ("Theme", "Тема"),
        ("Dark Theme", "Тёмная тема"),
        ("Dark", "Тёмная"),
        ("Light", "Светлая"),
        ("Follow System", "Системная"),
        ("Enable hardware codec", "Использовать аппаратный кодек"),
        ("Unlock Security Settings", "Разблокировать настройки безопасности"),
        ("Enable Audio", "Включить звук"),
        ("Unlock Network Settings", "Разблокировать настройки соединения"),
        ("Server", "Сервер"),
        ("Direct IP Access", "Прямой IP-доступ"),
        ("Proxy", "Прокси"),
        ("Apply", "Применить"),
        ("Disconnect all devices?", "Отключить все устройства?"),
        ("Clear", "Очистить"),
        ("Audio Input Device", "Источник звука"),
        ("Deny remote access", "Запретить удалённый доступ"),
        ("Use IP Whitelisting", "Использовать белый список IP"),
        ("Network", "Соединение"),
        ("Enable RDP", "Включить RDP"),
        ("Pin menubar", "Закрепить строку меню"),
        ("Unpin menubar", "Открепить строку меню"),
        ("Recording", "Запись"),
        ("Directory", "Папка"),
        ("Automatically record incoming sessions", "Автоматически записывать входящие сеансы"),
        ("Change", "Изменить"),
        ("Start session recording", "Начать запись сеанса"),
        ("Stop session recording", "Остановить запись сеанса"),
        ("Enable Recording Session", "Включить запись сеанса"),
        ("Allow recording session", "Разрешить запись сеанса"),
        ("Enable LAN Discovery", "Включить обнаружение в локальной сети"),
        ("Deny LAN Discovery", "Запретить обнаружение в локальной сети"),
        ("Write a message", "Написать сообщение"),
        ("Prompt", "Подсказка"),
        ("Please wait for confirmation of UAC...", "Дождитесь подтверждения UAC..."),
        ("elevated_foreground_window_tip", "Текущее окно удалённого рабочего стола требует более высоких привилегий для работы, поэтому временно невозможно использовать мышь и клавиатуру. Можно попросить удалённого пользователя свернуть текущее окно или нажать кнопку повышения прав в окне управления подключением. Чтобы избежать этой проблемы в дальнейшем, рекомендуется выполнить установку программного обеспечения на удалённом устройстве."),
        ("Disconnected", "Отключено"),
        ("Other", "Другое"),
        ("Confirm before closing multiple tabs", "Подтверждать закрытие несколько вкладок"),
        ("Keyboard Settings", "Настройки клавиатуры"),
        ("Full Access", "Полный доступ"),
        ("Screen Share", "Поделиться экраном"),
        ("Wayland requires Ubuntu 21.04 or higher version.", "Wayland требует Ubuntu 21.04 или более позднюю версию."),
        ("Wayland requires higher version of linux distro. Please try X11 desktop or change your OS.", "Для Wayland требуется более поздняя версия дистрибутива Linux. Пожалуйста, попробуйте рабочий стол X11 или смените ОС."),
        ("JumpLink", "Просмотр"),
        ("Please Select the screen to be shared(Operate on the peer side).", "Выберите экран для совместного использования (работайте на одноранговой стороне)."),
        ("Show RustDesk", "Показать RustDesk"),
        ("This PC", "Этот компьютер"),
        ("or", "или"),
        ("Continue with", "Продолжить с"),
        ("Elevate", "Повысить"),
        ("Zoom cursor", "Масштабировать курсор"),
        ("Accept sessions via password", "Принимать сеансы по паролю"),
        ("Accept sessions via click", "Принимать сеансы по нажатию"),
        ("Accept sessions via both", "Принимать сеансы по паролю+нажатию"),
        ("Please wait for the remote side to accept your session request...", "Подождите, пока удалённая сторона примет ваш запрос на сеанс..."),
        ("One-time Password", "Одноразовый пароль"),
        ("Use one-time password", "Использовать одноразовый пароль"),
        ("One-time password length", "Длина одноразового пароля"),
        ("Request access to your device", "Запрос доступа к вашему устройству"),
        ("Hide connection management window", "Скрывать окно управления соединениями"),
        ("hide_cm_tip", "Разрешать скрытие случае, если принимаются сеансы по паролю или используется постоянный пароль"),
        ("wayland_experiment_tip", "Поддержка Wayland находится на экспериментальной стадии, используйте X11, если вам требуется автоматический доступ."),
        ("Right click to select tabs", "Выбор вкладок щелчком правой кнопки мыши"),
        ("Skipped", "Пропущено"),
        ("Add to Address Book", "Добавить в адресную книгу"),
        ("Group", "Группа"),
        ("Search", "Поиск"),
        ("Closed manually by web console", "Закрыто вручную через веб-консоль"),
        ("Local keyboard type", "Тип локальной клавиатуры"),
        ("Select local keyboard type", "Выберите тип локальной клавиатуры"),
        ("software_render_tip", "Если у вас видеокарта Nvidia и удалённое окно закрывается сразу после подключения, может помочь установка драйвера Nouveau и выбор использования программной визуализации. Потребуется перезапуск."),
        ("Always use software rendering", "Использовать программную визуализацию"),
        ("config_input", "Чтобы управлять удалённым рабочим столом с помощью клавиатуры, необходимо предоставить RustDesk разрешения \"Мониторинг ввода\"."),
        ("request_elevation_tip", "Также можно запросить повышение прав, если кто-то есть на удалённой стороне."),
        ("Wait", "Ждите"),
        ("Elevation Error", "Ошибка повышения прав"),
        ("Ask the remote user for authentication", "Запросить аутентификацию у удалённого пользователя"),
        ("Choose this if the remote account is administrator", "Выберите это, если удалённый аккаунт является администратором"),
        ("Transmit the username and password of administrator", "Передать имя пользователя и пароль администратора"),
        ("still_click_uac_tip", "По-прежнему требуется, чтобы удалённый пользователь нажал \"OK\" в окне UAC при запуске RustDesk."),
        ("Request Elevation", "Запросить повышение"),
        ("wait_accept_uac_tip", "Подождите, пока удалённый пользователь подтвердит запрос UAC."),
        ("Elevate successfully", "Права повышены"),
        ("uppercase", "заглавные"),
        ("lowercase", "строчные"),
        ("digit", "цифры"),
        ("special character", "спецсимволы"),
        ("length>=8", "8+ символов"),
        ("Weak", "Слабый"),
        ("Medium", "Средний"),
        ("Strong", "Стойкий"),
        ("Switch Sides", "Переключить стороны"),
        ("Please confirm if you want to share your desktop?", "Подтверждаете, что хотите поделиться своим рабочим столом?"),
        ("Closed as expected", "Закрыто по ожиданию"),
        ("Display", "Отображение"),
        ("Default View Style", "Стиль отображения по умолчанию"),
        ("Default Scroll Style", "Стиль прокрутки по умолчанию"),
        ("Default Image Quality", "Качество изображения по умолчанию"),
        ("Default Codec", "Кодек по умолчанию"),
        ("Bitrate", "Битрейт"),
        ("FPS", "FPS"),
        ("Auto", "Авто"),
        ("Other Default Options", "Другие параметры по умолчанию"),
        ("Please confirm if you want to share your desktop?", "Подтвердите, что хотите поделиться своим рабочим столом?"),
        ("Closed as expected", ""),
        ("Guest to Host", ""),
        ("Dual way", ""),
        ("Audio Transmission Mode", ""),
    ].iter().cloned().collect();
}
