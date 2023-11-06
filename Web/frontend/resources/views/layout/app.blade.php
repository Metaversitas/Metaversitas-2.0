<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <meta http-equiv="X-UA-Compatible" content="ie=edge">
    <link href="https://cdn.jsdelivr.net/npm/bootstrap@5.3.2/dist/css/bootstrap.min.css" rel="stylesheet" integrity="sha384-T3c6CoIi6uLrA9TneNEoa7RxnatzjcDSCmG1MXxSR1GAsXEV/Dwwykc2MPK8M2HN" crossorigin="anonymous">
    <title>Document</title>
</head>
<body>
{{-- Codingan nvabar --}}
<nav class="navbar navbar-expand-lg p-3 " style="background-color:#FFE3E3; height: 80px">
<div class="container">
    <img src="/meta.png" alt="logo metaverse dan kampus merdeka" width="283px" >
    <div class="justify-content-end">
    <img src="/kamer.png" alt="logo metaverse dan kampus merdeka" width="163px" class="h-auto d-inline-block">
    </div>
  </div>
</nav>
<nav class="navbar navbar-expand-lg" style="background-color:#F8F8F8; height: 80px">
  <div class="container">
    <button class="navbar-toggler" type="button" data-bs-toggle="collapse" data-bs-target="#navbarNav" aria-controls="navbarNav" aria-expanded="false" aria-label="Toggle navigation">
    <span class="navbar-toggler-icon"></span>
    </button>
    <div class="collapse navbar-collapse" id="navbarNav">
        <ul class="navbar-nav">
        <li class="nav-item me-5">
          <a class="nav-link active" aria-current="page" href="#">Beranda</a>
        </li>
        <li class="nav-item me-5">
          <a class="nav-link" href="#">Program</a>
        </li>
        <li class="nav-item me-5">
          <a class="nav-link" href="#">Unduh</a>
        </li>
        <li class="nav-item me-5">
          <a class="nav-link" href="#">Tentang</a>
        </li>
        </ul>
            <div class="collapse navbar-collapse justify-content-end" id="navbarNav">
            <ul class="navbar-nav">
            <li class="nav-item me-4">
            <button type="button" class="btn text-black border border-secondary" style="">Daftar</button>
            </li>
            <li class="nav-item">
            <button type="button" class="btn text-black" style="background-color:#CACACA">Masuk</button>
            </li>
            </ul>   
            </div>
    </div>
  </div>
</nav>
</body>
@yield('navbar')
<script src="https://cdnjs.cloudflare.com/ajax/libs/bootstrap/5.3.2/js/bootstrap.min.js" integrity="sha512-WW8/jxkELe2CAiE4LvQfwm1rajOS8PHasCCx+knHG0gBHt8EXxS6T6tJRTGuDQVnluuAvMxWF4j8SNFDKceLFg==" crossorigin="anonymous" referrerpolicy="no-referrer"></script>

</html>