import open3d as o3d
import numpy as np

def main():
    # pcd = o3d.t.io.read_point_cloud("../assets/points.pcd")
    pcd = o3d.io.read_point_cloud("../assets/points.pcd")
    # print(pcd)
    # o3d.visualization.draw([pcd])
    
    # mpoints = np.array([

    # ])
    mpoints = []
    for point in pcd.points:
        # print([float(point[0]), float(point[1]), float(point[2]) - 220])
        mpoints.append([point[0], point[1], point[2] - 220])
        # point -= 220
        # print(point)
        # print(mpoints)
    # print("Hello from python-pcd!")
    pcd = o3d.geometry.PointCloud()
    pcd.points = o3d.utility.Vector3dVector(mpoints)
    o3d.io.write_point_cloud("hak_binary2.pcd", pcd, compressed=False, write_ascii=True)

if __name__ == "__main__":
    main()
