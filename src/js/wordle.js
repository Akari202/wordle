import {get_answer_words, get_allowed_words} from "/src/js/wordle_words";

let debug = true;

let word_length = 5;
let max_attempts = 6;
let allowed = [];
let words = [];
let game_board = [];
let game_area = document.getElementsByClassName("wordle")[0];
let win_modal = document.getElementById("win_modal");
let win_modal_copy = document.getElementsByClassName("copy")[0];
let win_modal_replay = document.getElementsByClassName("replay")[0];
let win_modal_game = document.getElementsByClassName("modal_game")[0];
let current_word;
let current_row = 0;
let current_guess;
let current_game = [];
let prev_game = "";
win_modal.style.display = "none";
allowed = get_allowed_words(word_length);
words = get_answer_words(word_length);

win_modal_replay.onclick = function () {
    win_modal.style.display = "none";
    init_game();
}

win_modal_copy.onclick = function () {
    navigator.clipboard.writeText(prev_game).then(function () {
        alert(prev_game + "Successfully saved to clipboard");
    }).catch(function () {
        alert(prev_game + "Failed to save to clipboard, something went wrong");
    });
}

function init_game() {
    current_word = words[Math.floor(Math.random() * words.length)];
    if (debug) {
        console.log(current_word);
    }
    let temp = document.getElementsByClassName("input");
    for (let i = 0; word_length * max_attempts > i; i++) {
        temp[i].value = "";
        temp[i].style.backgroundColor = "";
        temp[i].disabled = i >= word_length;
    }
    temp[0].focus();
}

function win_game() {
    let temp = word_length.toString() + " Letter Wordle " +
        words.indexOf(current_word).toString() + " " +
        current_game.length.toString() + "/" +
        max_attempts.toString() + "\n";
    for (let i = 0; current_game.length > i; i++) {
        let temp_row = current_game[i];
        for (let i1 = 0; word_length > i1; i1++) {
            temp = temp.concat(temp_row[i1]);
        }
        temp = temp.concat("\n");
    }
    console.log(temp);
    prev_game = temp;
    current_row = 0;
    current_game = [];
    win_modal_game.innerText = temp;
    win_modal.style.display = "block";
}

function lose_game() {
    let temp = word_length.toString() + " Letter Wordle Lost " +
        words.indexOf(current_word).toString() + "\n";
    for (let i = 0; current_game.length > i; i++) {
        let temp_row = current_game[i];
        for (let i1 = 0; word_length > i1; i1++) {
            temp = temp.concat(temp_row[i1]);
        }
        temp = temp.concat("\n");
    }
    temp = temp.concat(current_word);
    console.log(temp);
    prev_game = temp;
    current_row = 0;
    current_game = [];
    win_modal_game.innerText = temp;
    win_modal.style.display = "block";
}

function check_word() {
    let temp = document.getElementsByClassName("row" + current_row.toString());
    let temp_score = 0;
    let temp_game = [];
    current_guess = temp[0].children[0].value.toLowerCase();
    for (let i = 1; word_length > i; i++) {
        current_guess = current_guess.concat(temp[0].children[i].value.toLowerCase());
    }
    if (allowed.indexOf(current_guess) !== -1 || words.indexOf(current_guess) !== -1) {
        let temp_current_word = current_word;
        let temp_current_guess = current_guess;
        for (let i = 0; word_length > i; i++) {
            temp[0].children[i].disabled = true;
            if (current_guess.charAt(i) === current_word.charAt(i)) {
                temp_current_word = temp_current_word.replaceAt(i, " ");
                temp_current_guess = temp_current_guess.replaceAt(i, " ");
                temp[0].children[i].style.backgroundColor = "#6aaa64";
                temp_game[i] = "🟩";
                temp_score++;
            }
        }
        if (temp_score !== word_length) {
            for (let i = 0; word_length > i; i++) {
                if (temp_current_word.indexOf(temp_current_guess.charAt(i)) !== -1 &&
                    temp_current_guess.charAt(i) !== " ") {
                    temp_current_word = temp_current_word.replaceAt(
                        temp_current_word.indexOf(temp_current_guess.charAt(i)), " ");
                    temp_current_guess = temp_current_guess.replaceAt(i, " ");
                    temp[0].children[i].style.backgroundColor = "#c9b458";
                    temp_game[i] = "🟨️";
                    temp_score = 0;
                }
            }
            for (let i = 0; word_length > i; i++) {
                if (temp_current_guess.charAt(i) !== " ") {
                    temp_current_word = temp_current_word.replaceAt(
                        temp_current_word.indexOf(temp_current_guess.charAt(i)), " ");
                    temp_current_guess = temp_current_guess.replaceAt(i, " ");
                    temp[0].children[i].style.backgroundColor = "#787c7e";
                    temp_game[i] = "⬛";
                    temp_score = 0;
                }
            }
        }
        current_game[current_row] = temp_game;
        if (temp_score === word_length) {
            win_game();
        } else {
            current_row++;
            if (current_row !== max_attempts) {
                temp = document.getElementsByClassName("row" + current_row.toString());
                for (let i = 0; word_length > i; i++) {
                    temp[0].children[i].disabled = false;
                }
                temp[0].children[0].focus();
            } else {
                lose_game();
            }
        }
    } else {
        temp[0].style.animation = "shake .5s";
        setTimeout(function () {temp[0].style.animation = "";}, 500);
    }
}

for (let i = 0; max_attempts > i; i++) {
    let temp_list = [];
    let row_div = document.createElement("div");
    row_div.setAttribute("class", "row" + i.toString());
    row_div.setAttribute("tag", "row");
    for (let i1 = 0; word_length > i1; i1++) {
        let temp = document.createElement("INPUT");
        temp.setAttribute("class", "input");
        temp.setAttribute("maxlength", "1");
        temp.setAttribute("type", "text");
        temp.setAttribute("word_pos", i1.toString());
        temp.setAttribute("row_pos", i.toString());
        row_div.appendChild(temp);
        temp.addEventListener("keydown", function (event) {
            if (event.key === "Enter") {
                check_word();
            }
            else if (
                event.key === "Backspace" &&
                this !== row_div.firstChild &&
                this.value.length === 0) {
                this.previousElementSibling.focus();
            }
            else if (
                this.value.length === 1 &&
                this !== row_div.lastChild &&
                event.key !== "Backspace" &&
                event.key !== "Shift") {
                this.nextElementSibling.focus();
            }
        });
        temp.addEventListener("input", function () {
            this.value = this.value.toUpperCase();
        });
        if (i !== current_row) {
            temp.disabled = true;
        }
        temp_list[i1] = temp;
    }
    game_area.appendChild(row_div);
    game_board[i] = temp_list;
}

init_game();

String.prototype.replaceAt = function(index, replacement) {
    return this.substr(0, index) + replacement + this.substr(index + replacement.length);
}
