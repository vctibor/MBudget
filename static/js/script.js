window.onload = function(e) {
    
    // alert("Hello, World!")

    let categorySelects = document.getElementsByClassName("category_select");

    // alert(categorySelects.length);

    for (categorySelect in categorySelects) {

        let option = document.createElement("option");

        option.text = "Kiwi";
        
        categorySelect.add(option);

    }

    for (i = 0; i < categorySelects.length; i++) {
        categorySelects[i].style.backgroundColor = "red";
    }
}