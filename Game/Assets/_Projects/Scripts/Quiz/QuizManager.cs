using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using UnityEngine.UI;
using UnityEngine.IO;
using System.IO;
using TMPro;


public class QuizManager : MonoBehaviour
{
        public TextAsset assetSoal;

    private string[] soal;

    private string[,] soalBag;


    int indexSoal;
    int maxSoal;
    bool ambilSoal;
    char kunciJ;

    bool[] soalSelesai;

    public TMP_Text txtSoal, txtOpsiA, txtOpsiB, txtOpsiC, txtOpsiD;

    public bool isHasil;
    public bool isStart;

    int jwbBenar, jwbSalah;
    float  nilai;

    public GameObject Score;
    public GameObject imgHasil;
    public TMP_Text txtHasil;

    void Start()
    {
        soal = assetSoal.ToString().Split('#');

        soalSelesai = new bool[soal.Length];

        soalBag = new string[soal.Length, 6];
        maxSoal = soal.Length;
        OlahSoal();

        ambilSoal = true; 
        TampilkanSoal();

        print(soalBag[1,3]);

    }

    private void OlahSoal()
    {
        for (int i=0; i < soal.Length; i++)
        {
            string[] tempSoal = soal[i].Split('+');
            for(int j = 0; j < tempSoal.Length; j++)
            {
                soalBag[i, j] = tempSoal[j];
                continue;
            }
            continue;
        }
    }

    private void TampilkanSoal()
    {
        if (indexSoal<maxSoal)
        {
            if(ambilSoal)
            {
                for (int i = 0; i < soal.Length; i++)
                {
                    int randomIndexSoal = Random.Range(0, soal.Length);
                    if(!soalSelesai[randomIndexSoal])
                    {
                        txtSoal.text = soalBag[randomIndexSoal, 0];
                        txtOpsiA.text = soalBag[randomIndexSoal, 1];
                        txtOpsiB.text = soalBag[randomIndexSoal, 2];
                        txtOpsiC.text = soalBag[randomIndexSoal, 3];
                        txtOpsiD.text = soalBag[randomIndexSoal, 4];
                        kunciJ = soalBag[randomIndexSoal, 5][0];

                        soalSelesai[randomIndexSoal] = true;

                        ambilSoal = false;
                        break;
                    }
                    else
                    {
                        continue;
                    }
                }
            }
        }
    }

   
    public void Opsi(string opsiHuruf)
    {
       CheckJawaban(opsiHuruf[0]);
        
       if(indexSoal == maxSoal - 1)
        {
            isHasil = true;
        }
        else
        {
            indexSoal++;
            ambilSoal = true;
        }

        isStart = true;
    }

    private float HitungNilai()
    {
        return nilai = (float)jwbBenar / maxSoal * 100;
    }

    private void CheckJawaban(char huruf)
    {
        if (huruf.Equals(kunciJ))
        {
            jwbBenar++;
        }
        else
        {
            jwbSalah++;
        }

    }

    // Update is called once per frame
    void Update()
    {
        if(isStart)
        {
            if (isHasil)
            {
                
                Score.SetActive(true);
                txtHasil.text = "Jumlah benar : " + jwbBenar + "\nJumlah Salah : " + jwbSalah + "\n\nNilai : " + HitungNilai();
                // Put Saving Data To Json Function Here
            } 
            else
            {
                Score.SetActive(false);
                TampilkanSoal();
            }
        }
    }
}
