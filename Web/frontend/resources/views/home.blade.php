@extends('layout.app')
@section('navbar')
    <main>
        <div class="container mt-5">
                <div class="row">
                  <div class="col">
                    <h3 class="text-left">Belajar dengan cara yang lebih <Br>menyenangkan</h3>
                    <p class="text-left">Metaversitas adalah platform pembelajaran virtual berbasis Metaverse yang menghadirkan pengalaman pembelajaran yang seru dan interaktif bagi mahasiswa dan dosen. Dengan Metaversitas, pengguna dapat menjelajahi dunia praktikum secara virtual, menjalankan eksperimen, dan berinteraksi dengan materi pembelajaran secara mendalam, menjadikan pembelajaran lebih menarik dan efektif.</p>
                </div>
                <div class="col justify-content-end">
                      <img src="/mainsection.png" class="rounded-4" alt="Main section" width="545">
                  </div>
                </div>
        </div>
        <div class="container">
        <h2 class="text-center mt-5 mb-5">Program</h2>
        <div class="card-group">
            <div class="card border border-0 me-2" style="width: 15rem">
              <img src="/mainsection.png" class="card-img-top rounded-4" alt="...">
              <div class="card-body">
                <h5 class="card-title text-center">FISIKA</h5>
                <p class="card-text text-center">Kelas praktikum fisika merupakan suatu ruangan pembelajaran yang menggambarkan implementasi praktikum virtual secara 3D dari setiap modul-modul fisika seperti ayunan otomatis, Hukum Ohm, Archimedes, Katrol, dan masih banyak lagi.</p>
              </div>
            </div>
            
            <div class="card border border-0 me-2" style="width: 15rem">
              <img src="/mainsection.png" class="card-img-top rounded-4" alt="...">
              <div class="card-body">
                <h5 class="card-title text-center">Card title</h5>
                <p class="card-text text-center">This card has supporting text below as a natural lead-in to additional content.</p>
              </div>
            </div>
            
            <div class="card border border-0 me-2" style="width: 15rem">
              <img src="/mainsection.png" class="card-img-top rounded-4" alt="...">
              <div class="card-body">
                <h5 class="card-title text-center">Card title</h5>
                <p class="card-text text-center">This is a wider card with supporting text below as a natural lead-in to additional content. This card has even longer content than the first to show that equal height action.</p>
              </div>
            </div>
            </div>
           </div> 
          </div>

    <div class="container px-4 mt-5">
        <div class="row mx-auto">
          <div class="col">
           <div class="p-5">
            <img src="/mainsection.png" alt="Tentang Kami" width="500">
           </div>
          </div>
          <div class="col">
            <div class="pt-5 mt-5">
              <h3 class="text-left">Metaversitas</h3>
              <p class="text-left">Lorem ipsum dolor sit amet consectetur, adipisicing elit. Magni numquam explicabo nemo enim consequatur nam esse doloremque dolor voluptate eos, 
                sunt officiis saepe ea? Atque eum inventore culpa tempore doloremque.</p>
              <p class="text-primary">Petunjuk penggunaan</p>
              <div class="d-flex">
                <button type="button" class="btn btn-primary me-3">Unduh Untuk Windows</button>
                <button type="button" class="btn btn-primary">Unduh Untuk iOS</button>
            </div>
          </div>
        </div>
      </div>
    </div>
    @include('layout.footer')
    </main>
@endsection