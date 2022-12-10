window.onload = function() {
    let nickname_field = document.querySelector('#nickname')
    let password_field = document.querySelector('#password')
    let post_text = document.querySelector('#post_text')
    let status = document.querySelector('#status')
    let messages = [];
    let messages_container = document.querySelector('#messages_container');
    let page = 1;


    function update_messages() {
        document.querySelectorAll('.message').forEach(el => el.remove())
        for (let message of messages) {
            let element = document.createElement('p');
            element.innerText = message.text;
            element.classList.add('message')
            messages_container.appendChild(element)
        }
    }

    function load_messages() {
        fetch('http://localhost:8080/page', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
            },
            body: JSON.stringify({number: page})
        })
        .then(resp => resp.json())
        .then(resp => {
            messages = resp
            update_messages()
        })
    }

    function validate_nickname(nickname) {
        let result = true;
        for (let ch of nickname) {
            result &= ('a' <= ch && ch <= 'z') || ('A' <= ch && ch <= 'Z') || ('0' <= ch && ch <= '9')
        }
        return result;
    }

    let login_button = document.querySelector('#login')
    login_button.onclick = function() {
        let data = {
            'nickname': nickname_field.value,
            'password': password_field.value,
        }
        if (!validate_nickname(data.nickname)) {
            alert('Nickname should consist only of alphanumeric symbols');
            return;
        }
        nickname_field.value = '';
        password_field.value = '';
        fetch('http://localhost:8080/login', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
            },
            body: JSON.stringify(data),
        }).then(resp => {
            if (!resp.ok) {
                alert('wrong login or password')
            }
        })
    }

    let register_button = document.querySelector('#register')
    register_button.onclick = function() {
        
        let data = {
            'nickname': nickname_field.value,
            'password': password_field.value,
        }
        if (!validate_nickname(data.nickname)) {
            alert('Nickname should consist only of alphanumeric symbols');
            return;
        }
        nickname_field.value = '';
        password_field.value = '';
        fetch('http://localhost:8080/register', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
            },
            body: JSON.stringify(data)
        }).then(resp => {
            if (!resp.ok) {
                alert('this nickname already taken')
            }
        })
    }

    let post_button = document.querySelector('#post')
    post_button.onclick = function() {
        fetch('http://localhost:8080/post', {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json; charset=utf-8',
            },
            body: JSON.stringify({text: post_text.value})
        }).then(resp => {
            if (!resp.ok) {
                alert('you can\'t post without authorization')
            }
            load_messages()
        })
    }

    let next_button = document.querySelector('#next')
    next_button.onclick = function() {
        page += 1;
        load_messages()
    };

    let prev_button = document.querySelector('#previous')
    prev_button.onclick = function() {
        if (page != 1) {
            page -= 1;
            load_messages()
        }
    };

    load_messages()
}