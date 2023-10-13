import os
from gtts import gTTS

# 1
tts = gTTS('Relief Bhadracari bisa dilihat di dinding utama, lorong tingkat III dan IV, maupun pagar langkan di Candi Borobudur. Pahatan Bhadracari menceritakan usaha Sudhana untuk mencapai kebuddhaan, dengan berguru pada Boddhisatva Maitreya dan Boddhisatva Samanthabhadra.',
           lang='id')
tts.save("ReliefBhadracari.mp3")
# os.system('start ReliefBhadracari.mp3')

# # 2
# tts = gTTS('Yaś-ca imaṁ pariṇāmana-Rājaṁ rutva, sakṛj-janayed-adhimuktim, Bodhi-varām-anŭprārthayamāno, agru viśiṣṭa bhaved-imu puṇyam; Semoga dia, setelah mendengar Raja pemenuhan sempurna ini, segera mengerti; bertekad, menginginkan Kebangunan mulia, semoga jasa ini menjadi istimewa dan menonjol (Syair 48). Relief seri ini memang Gaṇḍavyūha, dan relief terakhir yang terlihat sebelum naik ke teras, di atas menunjukkan sembilan Buddha di daftar teratas dan delapan di bawah, dengan Samantabhadra dan Sudhana, keduanya sekarang digambarkan duduk di atas teratai.',
#            lang='id')
# tts.save("72TujuhBelasBuddha.mp3")
# # os.system('start 72TujuhBelasBuddha.mp3')
