<a name="v1.2.1"></a>
##  (2016-01-05)


#### Improvements

*   Reimplementation of field generation to avoid game over on first click after game beginning ([0723d12c](https://github.com/Vinatorul/minesweeper-rs/commit/0723d12c42c78c169723414ad72b5dd3575a0163))



<a name="v1.2"></a>
##  (2015-10-26)


#### Improvements

*   Display game result on GUI. ([e99e267f](https://github.com/Vinatorul/minesweeper-rs/commit/e99e267f33abdd071a405ba4337d5cce7ab37557))

*	Implement redrawing when it needs for fixing. ([9793bc38](https://github.com/Vinatorul/minesweeper-rs/commit/9793bc38041836098792bb55e210d40b6583e43a))

#### Features

*   added the timer ([eaecc403](https://github.com/Vinatorul/minesweeper-rs/commit/eaecc4035720e1c47c3a1591745a60525edb7395))



<a name="v1.1"></a>
##  (2015-09-07)


#### Features

*   added a mine counter ([d2906e9e](https://github.com/Vinatorul/minesweeper-rs/commit/d2906e9e39794fca338adc6536b8bac05276ec21))
*   colored numbers ([eaf61bbc](https://github.com/Vinatorul/minesweeper-rs/commit/eaf61bbce8a8950a9ca856f3f5c2dc2c8bac4c59))
*   colored numbers ([b527f805](https://github.com/Vinatorul/minesweeper-rs/commit/b527f8056564b610949c25aab109dfbfb5ef8af0))
*   show the user which mine ended the game ([b2168f01](https://github.com/Vinatorul/minesweeper-rs/commit/b2168f01d3591b275fb48adb5c16b541f4014541))
*   when you click on a revealed number, it clicks on unrevealed neighbors as a shortcut. ([67942f5c](https://github.com/Vinatorul/minesweeper-rs/commit/67942f5cf8f701f851c3a282139838bf4dbb219d))
* **Game:**  Add ability to set max fps ([decbf729](https://github.com/Vinatorul/minesweeper-rs/commit/decbf729cf757ee65091d4716797e97d88de48d1))

#### Bug Fixes

*   minor bugs fixes ([6d04d05e](https://github.com/Vinatorul/minesweeper-rs/commit/6d04d05e32e65e8030f59d049cc42bbb13ef92cb))
*   fixed problems with OpenGL version ([0df987b8](https://github.com/Vinatorul/minesweeper-rs/commit/0df987b856bbd7aeb92a68ef28c8ca02cfb49127))
*   true number of mines on board, not the case before ([0eaad50e](https://github.com/Vinatorul/minesweeper-rs/commit/0eaad50ef2536e07dae9d28df31499c95b35c946))
*   fixed problems with OpenGL version ([c370245c](https://github.com/Vinatorul/minesweeper-rs/commit/c370245cb4fdcdd88bfbd4c8e4edf41add08f308))
*   fixed glutin 48 error ([7b45d2b3](https://github.com/Vinatorul/minesweeper-rs/commit/7b45d2b3f56aadb50f4c3e9e6ecce8a02ae30ff6))
* **game:**  went out of bounds in some cases during neighbor check ([95b61f3b](https://github.com/Vinatorul/minesweeper-rs/commit/95b61f3b7d6989143cf968f31fea6fc079af3bd8))



<a name="v1.0"></a>
## v1.0 (2015-08-30)


#### Features

* **Game:**
  *  add config via command line arguments ([05736cc5](https://github.com/Vinatorul/minesweeper-rs/commit/05736cc5538e248ef300912c9506c8fe72d858f9))
  *  added ability to mark cells ([3cb91ea4](https://github.com/Vinatorul/minesweeper-rs/commit/3cb91ea45e18d21ed81da0c56812dc5053d88aba))
  *  Added grid ([489ba90a](https://github.com/Vinatorul/minesweeper-rs/commit/489ba90acfdb5afe7f08b3e4418aea46b76e5657))
* **UI:**
  *  add ability to change settings ([12c8ead9](https://github.com/Vinatorul/minesweeper-rs/commit/12c8ead97c0aab1a15da3d1e2615e3281cefcb4f))
  *  added ui drawing ([e8c81d20](https://github.com/Vinatorul/minesweeper-rs/commit/e8c81d20cf24dfe5781ce1d9771911553459dacf))

#### Improvements

* **Field:**  allows field to fly ([730fac12](https://github.com/Vinatorul/minesweeper-rs/commit/730fac121f1dfbce530b6c835909dbd1f2b52028))
* **Game:**  added ability to win ([568b3a9c](https://github.com/Vinatorul/minesweeper-rs/commit/568b3a9c5ad4f43574ecf4d6c0e13d2745ade22c))



<a name="v0.1.0"></a>
## v0.1.0 (2015-08-27)


#### Improvements

* **Field:**  now clicking on empty cell will reveal all nearby cells ([d5dae61f](https://github.com/Vinatorul/minesweeper-rs/commit/d5dae61fc8922e1a9a0f933f56daba891ef6cb1f))
* **Game, Field:**  massive code improvements ([f4359660](https://github.com/Vinatorul/minesweeper-rs/commit/f4359660659dd88b3e755e2a2dbc6829f5551a95))