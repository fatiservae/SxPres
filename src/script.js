var slides = document.querySelectorAll('.slide');
var currentslideIndex = 0;
var n = 0;

const popup = document.getElementById("popup");
const popupText = document.getElementById("conteudo-popup");

function popUpShow(){
    popupText.innerHTML = `
                 <h1>Press <em>"t"</em><br> to quit this help</h1>
                 <h2>Slide ${currentslideIndex+1} of ${slides.length}</h2>
                 <table>
                 <tr>
                   <th>Command</th><th>Key</th>
                 </tr>
                 <tr>
                   <td>Next</td><td>j</td>
                 </tr>
                 <tr>
                   <td>Previous</td><td>k</td>
                 </tr>
                 <tr>
                   <td>First</td><td>gg</td>
                 <tr>
                   <td>Last</td><td>G</td>
                 </tr>
                 <tr>
                   <td>Print</td><td>p</td>
                 </tr>
                 </table>
                 <p> On mobile, swipe with landscape.</p>
               `;
}
popUpShow();

function Printer() {
  for (var j = 0; j < slides.length; j++) {
      slides[j].style.display= 'block';
  }
};

document.addEventListener('keydown', function(event) {
  if (event.key === 'ArrowRight' || event.key === 'j') {
    if (currentslideIndex < slides.length - 1){
      var counter = parseInt(n);
      if (counter === 0) {counter = counter + 1}
      else if (counter > slides.length) {counter = 0};
      currentslideIndex = currentslideIndex + counter;
      n = 0;
    }
  }
  else if (event.key === 'ArrowLeft' || event.key === 'k'){
    if (currentslideIndex > 0) {
     var counter = parseInt(n);
     if (counter === 0) {counter = counter + 1};
     currentslideIndex = currentslideIndex - counter;
     if (currentslideIndex < 0 ){currentslideIndex = 0};
     n = 0;
    }
  }
  else if (event.key === 'p') {
    Printer();
    return;
  }
  else if (event.key === 't') {
    if (popup.style.display === 'none') {popup.style.display = "block"}
    else {popup.style.display = "none"};
    popUpShow()
  }
  else if (typeof event.key === 'string' && event.key >= '0' && event.key <= '9'){
    if (n === 0) {n = `${event.key}`}else{n = `${n}${event.key}`}
  }

  else if (event.key === 'g'){
    document.addEventListener('keydown', function(event) {
       if (event.key === 'g') {currentslideIndex = 0};
    })
  }
  
  else if (event.key === 'G') { currentslideIndex = slides.length - 1}

  else if (event.key === 'm'){toggleMovement()}

  else if (isMoving && event.key === 'ArrowUp' 
    || event.key === 'ArrowLeft' 
    || event.key === 'ArrowDown' 
    || event.key === 'ArrowRight') {moveCircle(event.key)}

  else if (event.key === 'x') {resizeMarker()}
  
  else if (event.key === '-') {resizeFont()}

  else {return};

  for (var i = 0; i < slides.length; i++) {
    if (i === currentslideIndex) {
      slides[i].style.display = 'flex';
    } else {
      slides[i].style.display = 'none';
    }
  }
});

const circle = document.getElementById('marcador');
var sizeMarker = 1;
let circleTop = 0;
let circleLeft = 0;
let isMoving = false;

function updateCirclePosition() {
  circle.style.top = circleTop + 'px';
  circle.style.left = circleLeft + 'px';
}

function resizeMarker() {
  if (sizeMarker > 250) {sizeMarker = 1} 
  else {sizeMarker = sizeMarker + 5};
  circle.style.width = sizeMarker + 'px';
  circle.style.height = sizeMarker + 'px';
}

document.addEventListener('mousemove', function(event) {
  const x = event.clientX;
  const y = event.clientY;
  marcador.style.transform = `translate(${x}px, ${y}px)`;
});

function moveCircle(direction) {
  const step = 10; 
  switch (direction) {
    case 'ArrowDown':
      circleTop += step;
      break;
    case 'ArrowUp':
      circleTop -= step;
      break;
    case 'ArrowLeft':
      circleLeft -= step;
      break;
    case 'ArrowRight':
      circleLeft += step;
      break;
  }

  updateCirclePosition();
}

function toggleMovement() {
  isMoving = !isMoving;
  if (isMoving) {
    circle.style.display = 'block';
  } else {
    circle.style.display = 'none';
  }
}
//function resizeFont() {
//    var elementosSlide = document.querySelectorAll(".slide");
//    if (elementoSlide) {
//        var novoFont = "16px";
//        elementoSlide.style.fontSize = novoFont;
//    } else {
//        console.error("Elemento 'slide' não encontrado.");
//    }
//}

// Executa uma tecl 'k' para iniciar layout correta
//document.addEventListener('DOMContentLoaded', function() {
//  const event = new KeyboardEvent('keydown', {
//    key: 'k',
//    keyCode: 75,
//    code: 'KeyK',
//    which: 75,
//    shiftKey: false, 
//    ctrlKey: false, 
//    altKey: false, 
//    metaKey: false, 
//  });
//  
//  document.dispatchEvent(event);
//});
