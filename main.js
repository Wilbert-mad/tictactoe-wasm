import init, { getState, takePlayerCell, nextTurn, checkPlayerWin, getTurn, clearGame } from './pkg/tictactoe_wasm.js';

init().then(() => {
  const root = document.body;

  function render() {
    root.innerHTML = '';
    root.classList.add('game_main', 'game_size');

    const state = getState();

    state.split('\n').map(function (row, rowIndex) {
      row
        .trim()
        .split(/\s+/)
        .map(function (cell, celIndex) {
          const cellElement = document.createElement('a');

          cellElement.classList.add('game_cell');
          cellElement.href = '#';
          cellElement.innerText = cell.toString();

          function handler(e) {
            e.preventDefault();

            // console.log(`Taking: (${rowIndex}, ${celIndex})`);

            if (!takePlayerCell(rowIndex, celIndex)) {
              alert('This cell is taken');
              return;
            }

            if (checkPlayerWin()) {
              root.innerHTML = '';
              root.classList.remove('game_main');

              const main = document.createElement('div');
              main.innerText = getTurn() == 0 ? '🟥 won' : '🟪 won';

              const newGame = document.createElement('div');

              newGame.style.cursor = 'pointer';
              newGame.style.color = 'blue';
              newGame.style.backgroundColor = '#e7e7e785';
              newGame.style.borderRadius = '3px';

              newGame.innerText = 'Start new game';
              newGame.addEventListener(
                'click',
                function () {
                  clearGame();
                  render();
                },
                { once: true }
              );

              root.appendChild(main);
              root.appendChild(newGame);

              return;
            }

            nextTurn();
            render();
          }

          cellElement.addEventListener('click', handler);
          root.appendChild(cellElement);
        });
    });
  }

  render();
});
