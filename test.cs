//
using UnityEngine;
using System.Collections;
//
public class test : MonoBehaviour
{
    public Mesh rock_5;

    void Start ()
    {
        ColladaExporter export = new ColladaExporter("rock_5.dae", true);
        export.AddGeometry("rock_5", rock_5);
        export.AddGeometryToScene("rock_5", "Rock");
        export.Save();
    }

}
