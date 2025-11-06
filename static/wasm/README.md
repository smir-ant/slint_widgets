# Custom slint widget set


<details>
  <summary><h3>HR</h3></summary>

  Just divider

<table>
<tr>
  <th>Result</th>
  <th>Code</th>
</tr>
<tr>
  <td><img width="300" src="https://github.com/user-attachments/assets/be7035d5-08a2-4eb3-9f27-da265e6c4013"></td>
  <td>

    
  ```slint
  Text { text: "Lorem ipsum"; }
  HR {}
  Text { text: "Lorem ipsum"; }
  HR {
      hr-width: 75%;
      hr-height: 5px;
      hr-background: Palette.accent-background;
      hr-border-radius: 5px;
      vertical-padding: 15px;
  }
  Text { text: "Lorem ipsum"; }
  ```
  </td>  
</tr>
</table>
</details>


<details>
  <summary><h3>Label border</h3></summary>

  A frame around the objects with a caption. Any object can be inside. Any number of objects inside.

<table>
<tr>
  <th>Result</th>
  <th>Code</th>
</tr>
<tr>
  <td><img width="300" src="https://github.com/user-attachments/assets/4a5ae70a-c001-4b73-a33a-dc18b895137e"></td>
  <td>
    
  ```slint
  LabelBorder {
    Text { text: "One two three four five"; }
  }
  ```
  </td>  
</tr>
<tr>
  <td><img width="300" src="https://github.com/user-attachments/assets/0f055544-8957-492b-93fd-8d51a750f8d2"></td>
  <td>
    
  ```slint
  LabelBorder {
      title: "Some label";
      Button { text: "Lorem ipsum"; }
  }
  ```
  </td>  
</tr>
<tr>
  <td><img width="300" src="https://github.com/user-attachments/assets/ff5be355-7d37-4006-b9c4-6fc86945abe0"></td>
  <td>
    
  ```slint
  LabelBorder {
      title: "* Your Age";
      LineEdit { placeholder-text: "Please enter your age"; }
      Text { 
          text: "Only 18+ \nThe administration can check your age.";
          font-size: 10px;
          color: Palette.foreground.with-alpha(0.4);
      }
  }
  ```
  </td>  
</tr>
<tr>
  <td><img width="300" src="https://github.com/user-attachments/assets/eaf6c7c1-5e69-4d14-ac63-a3c962229d8d"></td>
  <td>

    
  ```slint
  LabelBorder {
    title: "Choose the answer";
    width: 100%;  // full width, not fit-content
    label-font-size: 20px;
    border-radius: 15px;
    HorizontalLayout {
        spacing: 10px;
        for letter[indx] in ["A", "B", "C"]: Button {
            text: "\{letter} : \{indx}";
        }
    }
  }
  ```
  </td>  
</tr>
</table>
</details>



<details>
  <summary><h3>Dropdown</h3></summary>

  Drop-down menu. An analog of the `<details>` tag from html. There can be any number of elements inside.

<table>
<tr>
  <th>Result</th>
  <th>Code</th>
</tr>
<tr>
  <td><img width="300" src="https://github.com/user-attachments/assets/62ef343c-cb47-466b-8963-9bc975316601"></td>
  <td>

  ```slint
  Dropdown {
    title: "Question?";
    Text { text: "Thank you for your question!"; }
  }
  ```
  </td>  
</tr>
<tr>
  <td><img width="400" src="https://github.com/user-attachments/assets/4e319447-f753-4e17-9437-204d39afe7ab"></td>
  <td>

  ```slint
  Dropdown {
    title: "Click to expand";
    title-font-size: 20px;
    border-radius: 18px;
    header-background: true;
    expanded: true;
    width: 100%;

    LineEdit { placeholder-text: "Element number 1"; }
    Button { text: "Second(after first)"; }
    Text { text: "Third is bronze"; }
  }
  ```
  </td>  
</tr>
</table>
</details>



<details>
  <summary><h3>WheelPicker</h3></summary>

  Selection via scroll wheels. Magnet adjusted to the nearest value.
  Swipe and scrolling are supported!

<table>
<tr>
  <th>Result</th>
  <th>Code</th>
</tr>
<tr>
  <td><img width="400" src="https://github.com/user-attachments/assets/da716053-c09a-45e7-9da0-cba777a97d28"></td>
  <td>

  ```slint
  property <int> selected-hour: 0;
  property <int> selected-minute: 0;
  property <int> selected-second: 0;
  
  property <[string]> hours-model: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23",];
  property <[string]> minutes-model: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",];
  property <[string]> seconds-model: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",];
  
  VerticalLayout {
    HorizontalLayout {
        alignment: center;
        spacing: 10px;

        WheelPicker {
            sections: 5;
            model: hours-model;
            selected(index) => { selected-hour = index; }
        }
        WheelPicker {
            sections: 5;
            model: minutes-model;
            selected(index) => { selected-minute = index; }
        }
        WheelPicker {
            sections: 5;
            model: seconds-model;
            selected(index) => { selected-second = index; }
        }
    }

    Button {
        text: "Show time";
        clicked => {
            debug("\{selected-hour} : \{selected-minute} : \{selected-second}");
        }
    }
  }
  ```
  </td>  
</tr>
<tr>
  <td><img width="400" src="https://github.com/user-attachments/assets/46f4fe4e-1173-4c94-ad41-6212097c3317"></td>
  <td>


  ```slint
  property <int> selected-day: 0;
  property <int> selected-hour: 0;
  property <int> selected-minute: 0;
  
  property <[string]> dow-model: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
  property <[string]> hours-model: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23"];
  property <[string]> minutes-model: ["0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16", "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32", "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48", "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59",];
  
  VerticalLayout {
    spacing: 20px;
    HorizontalLayout {
        alignment: center;
        spacing: 10px;

        VerticalLayout {
            WheelPicker {
                width: 50px;
                sections: 3;
                reverse-scroll: true;
                accent-color: gray.with-alpha(0.3);
                model: dow-model;
                selected(index) => { selected-day = index; }
            }
            HorizontalLayout { alignment: center; Text { text: "Day"; font-size: 14px; }} 
        }
        VerticalLayout {
            WheelPicker {
                width: 150px;
                sections: 5;
                model: hours-model;
                reverse-scroll: true;
                accent-color: red;
                selected(index) => { selected-hour = index; }
            }
            HorizontalLayout { alignment: center; Text { text: "Hour"; font-size: 14px; }} 
        }
        VerticalLayout {
            WheelPicker {
                sections: 2;
                model: minutes-model;
                selected(index) => { selected-minute = index; }
            }
            HorizontalLayout { alignment: center; Text { text: "Minute"; font-size: 14px; }} 
        }
    }

    Button {
        text: "Show";
        clicked => {
            debug("\{dow-model[selected-day]} \{selected-hour} : \{selected-minute}");
        }
    }
  }
  ```
  </td>  
</tr>
</table>
</details>