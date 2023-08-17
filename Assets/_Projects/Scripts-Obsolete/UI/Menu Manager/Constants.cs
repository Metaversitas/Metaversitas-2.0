using System.Collections;
using System.Collections.Generic;
using UnityEngine;
using System;

namespace Metaversitas.Constants
{

    [Serializable]
    public enum Lobby
    {
        AYUNAN_LOBBY,
        ARCHIMEDES_LOBBY,
        VISCOSITY_LOBBY,
        HOOK_LOBBY,
        OHM_LOBBY,
        KATROL_LOBBY,
        BIDANG_MIRING_LOBBY,
        RLC_LOBBY,
        KIRCHOFF_LOBBY,
        PEMBIASAN_LOBBY,
        BOROBUDUR_LOBBY,
        META_LAND_LOBBY
    }

    [Serializable]
    public enum Enum_Menu
    {
        Loading,
        Modul,
        Main_Menu,
        Create_Lobby,
        Room_Lobby,
        Error,
        Finding_Room,
        Setting,
        Front_Page,
        Meta,
        Candi,
        Pause,
        Pause_Laporan,
        Pause_Setting,
        Pause_Exit,
        Laporan_Bandul,
        Splash,
    }

    [Serializable]
    public enum Background_Enum
    {
        Ayunan,
        Archimedes,
        Viscosity,
        Hook,
        Ohm,
        Katrol,
        Bidang_Miring,
        RLC,
        Kirchoff,
        Pembiasan,
        Borobudur,
        Meta_Land
    }

    [Serializable]
    public class Room
    {
        public const string IS_ROOM_STARTED = "isRoomStarted";
    }

    [Serializable]
    public enum Enum_Scene
    {
        Startup,
        Bandul,
        Borobudur,
        MetaLand,
        Archimedes,
        Ohm,
        Katrol,
        Viscositas,
        Hook
    }

    public class User {
        public const string NPM = "npm";
        public const string Nama = "nama";
        public const string Perguruan_Tinggi = "nama_pt";
        public const string Prodi = "nama_prodi";
        public const string Nickname = "nickname";
        public const string Kelamin = "kelamin";
        public const string Is_Dosen = "is_dosen";

        public enum Jenis_Kelamin
        {
            Laki_Laki,
            Perempuan,
        }
    }
    
}
